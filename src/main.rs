use std::{cell::LazyCell, error::Error, io::{self, stdout}, sync::{Arc, LazyLock, Mutex, atomic::{AtomicBool, Ordering}}, time::Duration};

use crate::{fft::FFT::{process_mono_audio, process_stereo_audio}, helper::Helper::CLI, model::Model::{MyColor, expand_to_len}, render::render::App};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}};
use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Terminal, backend::CrosstermBackend};
use ringbuf::{HeapRb, traits::{Observer, Producer, Split}};
mod helper;
mod model;
mod fft;
mod render;
pub const FRAME_SIZE:usize = 1024;

static running:LazyLock<Arc<AtomicBool>> = LazyLock::new(|| Arc::new(AtomicBool::new(true)));
static r: LazyLock<Arc<AtomicBool>> = LazyLock::new(|| running.clone());

pub fn add_sigint_handler(){
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).unwrap();
}

#[derive(Debug)]
struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>>{
    // Handle CTRL+C
    add_sigint_handler();
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal(); 
        original_hook(panic_info);
    }));

    let mut clargs = CLI::new();
    clargs.Parse_Args();

    if clargs.dbg{
        println!("{clargs:?}");
    }

    match clargs.colors.len(){
        0 => {clargs.colors.append(&mut vec![MyColor::new(255, 0, 0, 255),MyColor::new(0, 0, 255, 255)]);},
        1 => {clargs.colors.push(MyColor::new(255, 0, 0, 0));},
        _ => {}
    }    
    if clargs.to_invert{
        clargs.colors = expand_to_len(clargs.colors, clargs.bars).iter().map(|x| MyColor::Invert(x)).collect();
    }else{
        clargs.colors = expand_to_len(clargs.colors, clargs.bars);
    }

    let mut app = App::new(clargs.bars, clargs.amp as f32, clargs.colors, clargs.stereo);
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout,EnterAlternateScreen,EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut terminal = TerminalGuard {terminal};
    
    let host = cpal::host_from_id(render::render::select_host(&mut terminal.terminal)?)?;
    let device_name = render::render::select_device(&mut terminal.terminal, &host)?;
    
    let mut stream = None;
    let mut sample_rate = 48000_u32;
    let mut channels = 2;
    let RING_BUFF_SZ = sample_rate as usize * 2;
    let mut proc_buff = HeapRb::<f32>::new(RING_BUFF_SZ);

    let (mut lbuff,mut rbuff) = (HeapRb::<f32>::new(RING_BUFF_SZ/2),HeapRb::<f32>::new(RING_BUFF_SZ/2));
    let (mut lprod,mut lcons) = lbuff.split();
    let (mut rprod,mut rcons) = rbuff.split();
    let (mut prod,mut cons) = proc_buff.split();

    let lcons = Arc::new(Mutex::new(lcons));
    let rcons = Arc::new(Mutex::new(rcons));
    let cons = Arc::new(Mutex::new(cons));

    for dev in host.input_devices().unwrap(){       
        let name = dev.to_string();
        if name == device_name {
            let conf = dev.default_input_config().expect("No config for the input device");
            let conf: cpal::StreamConfig = conf.clone().into();
            sample_rate = conf.sample_rate;
            channels = conf.channels;
            assert!(channels <= 2);
            stream = Some(dev.build_input_stream(conf, move |data:&[f32],_|{
                if clargs.stereo{
                    for (idx,v) in data.iter().enumerate(){
                        if idx & 1 == 0{
                            let _ = lprod.try_push(*v);
                        }else{
                            let _ = rprod.try_push(*v);
                        }
                    }
                } else{
                    for i in data.chunks_exact(2){
                        let _ = prod.try_push((i[0] + i[1]) * 0.5);
                    }
                }
            }, move |err| {panic!("Exiting due to {err}");}, None).unwrap());
            break;
        }
    }

    if let Some(strm) = stream{
        let app = Arc::new(Mutex::new(app));

        let app_clone = app.clone();
        if clargs.stereo{
            let lcons_clone = lcons.clone();
            let rcons_clone = rcons.clone();
            std::thread::spawn(move || {process_stereo_audio(app_clone, lcons_clone, rcons_clone, sample_rate, channels);});
        }else{
            let cons_clone = cons.clone();
            std::thread::spawn(move || {process_mono_audio(app_clone, cons_clone, sample_rate, channels);});
        }
        
        strm.play()?;
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout,EnterAlternateScreen,EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let mut terminal = TerminalGuard {terminal};
        render::render::run_app(&mut terminal.terminal, app)?;
        Ok(())
        
    }else{
        panic!("No valid virtual stream found on host device");
    }
}
