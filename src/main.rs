use std::{error::Error, time::Duration};

use crate::{fft::FFT::{process_mono_audio, process_stereo_audio}, helper::Helper::CLI, model::Model::{CaptureSource, MyColor, expand_to_len}};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}};
use ringbuf::{HeapRb, traits::{Producer, Split}};
mod helper;
mod model;
mod fft;
mod render;
pub const FRAME_SIZE:usize = 1024;

fn main() -> Result<(), Box<dyn Error>>{
    let mut clargs = CLI::new();
    clargs.Parse_Args();

    if clargs.dbg{
        println!("{clargs:?}");
    }

    let src = if let Some(x) = clargs.src{ CaptureSource::from(&x[..])} else { CaptureSource::DefaultInput};

    match clargs.colors.len(){
        0 => {clargs.colors.append(&mut vec![MyColor::new(255, 0, 0, 255),MyColor::new(0, 0, 255, 255)]);},
        1 => {clargs.colors.push(MyColor::new(255, 0, 0, 0));},
        _ => {}
    }    


    clargs.colors = expand_to_len(clargs.colors, clargs.bars);
    println!("Available Hosts: ");
    for i in cpal::available_hosts(){
        println!("{}",i.name());
    }
    let host = cpal::default_host();
    let mut stream = None;
    println!("Available Input Streams: ");
    let mut sample_rate = 48000_u32;
    let mut channels = 2;
    let mut RING_BUFF_SZ = sample_rate as usize * 2;
    let mut proc_buff = HeapRb::<f32>::new(RING_BUFF_SZ); // Store 2secs of intervieved L R frames

    let (mut Lbuff,mut Rbuff) = (HeapRb::<f32>::new(RING_BUFF_SZ/2),HeapRb::<f32>::new(RING_BUFF_SZ/2));
    let (mut lprod,mut lcons) = Lbuff.split();
    let (mut rprod,mut rcons) = Rbuff.split();

    let (mut prod,mut cons) = proc_buff.split();
    match src{
    CaptureSource::DefaultInput => {
        for dev in host.input_devices().unwrap(){       
            let name = dev.name().unwrap(); 
            println!("{}",&name);
            if matches!(&name[..],"default"){
                let conf = dev.default_input_config().expect("No config for the input device");
                let conf: cpal::StreamConfig = conf.clone().into();
                sample_rate = conf.sample_rate.0;
                channels = conf.channels;
                stream = Some(dev.build_input_stream(&conf, move |data:&[f32],_|{
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
    },
    CaptureSource::PipeWireMonitor => {
        for dev in host.input_devices().unwrap(){        
            let name = dev.name().unwrap();
            let lwr = name.to_lowercase();
            println!("{}",&name);
            if lwr.contains("monitor") && lwr.contains("pipewire") && lwr.contains("output"){
                let conf = dev.default_input_config().expect("No config for the input device");
                let conf: cpal::StreamConfig = conf.clone().into();
                sample_rate = conf.sample_rate.0;
                channels = conf.channels;
                stream = Some(dev.build_input_stream(&conf, move |data:&[f32],_|{
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
    },
    CaptureSource::NamedDevice(s) => {
        for dev in host.input_devices().unwrap(){        
            let name = dev.name().unwrap();
            println!("{}",&name);
            if &name[..] == &s[..]{
                let conf = dev.default_input_config().expect("No config for the input device");
                let conf: cpal::StreamConfig = conf.clone().into();
                sample_rate = conf.sample_rate.0;
                channels = conf.channels;
                stream = Some(dev.build_input_stream(&conf, move |data:&[f32],_|{
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
    }
    }
    if let Some(strm) = stream{
        strm.play()?;
        if clargs.stereo{
            std::thread::spawn(move ||{process_stereo_audio(&mut lcons,&mut rcons, sample_rate, channels);});
        }else{
            std::thread::spawn(move ||{process_mono_audio(&mut cons,sample_rate, channels);});
        }
    }else{
        panic!("No valid virtual stream found on host device");
    }

    loop{
        std::thread::sleep(Duration::from_secs(1));
    }
}
