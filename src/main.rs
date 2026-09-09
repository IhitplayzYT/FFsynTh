use crate::{helper::Helper::CLI, model::Model::{MyColor, expand_to_len}};
use cpal::{FromSample, StreamConfig, traits::{DeviceTrait,HostTrait}};
use rustfft::num_traits::clamp;
mod helper;
mod model;
fn main() {
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


    clargs.colors = expand_to_len(clargs.colors, clargs.bars);
    let host = cpal::default_host();
    println!("Available Input Streams: ");
    for dev in host.input_devices().unwrap(){        
        if matches!(&dev.name().unwrap()[..],"default" | "pipewire"){
            let conf = dev.default_input_config().expect("No config for the input device");
            let conf: cpal::StreamConfig = conf.clone().into();
            let stream = dev.build_input_stream(&conf, move |data:&[f32],_|{


                
                
            }, move |err| {panic!("Exiting due to {err}");}, None).unwrap();
            break;
        }

    }



}
