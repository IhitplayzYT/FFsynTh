pub mod Model{
    use ringbuf::traits::Consumer;

use crate::model::Model::CaptureSource::NamedDevice;


    #[derive(Debug,Clone, Copy)]
  pub struct MyColor{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
  }  

  impl Default for MyColor{
    fn default() -> Self {
        Self { r: 255, g: 255, b: 255, a: 230 }
    }

  }

  impl MyColor {
    pub fn new(r:u8,g:u8,b:u8,a:u8) -> Self{
        Self { r, g, b, a }
    }

    pub fn invert(&mut self){
        self.r = u8::MAX - self.r;
        self.g = u8::MAX - self.g;
        self.b = u8::MAX - self.b;
    }
  }


  impl From<&str> for MyColor{
    fn from(value: &str) -> Self {
        // (a,b,c,d)
        let value = value.trim_end_matches(")");
        let channels = value.split(",").map(|x| x.trim().parse().expect("each component is a byte btwn 0-255")).collect::<Vec<u8>>();
        match channels.len(){
            3 => {
                MyColor { r: channels[0], g: channels[1], b: channels[2], a: 255 }
            },
            4 => {
                MyColor { r: channels[0], g: channels[1], b: channels[2], a: channels[3]}
            }
            _ => {panic!("Invalid str passsed")}
        }
    }
  }


  pub fn expand_to_len(mut colors: Vec<MyColor>, target_len: usize) -> Vec<MyColor> {
    if colors.len() >= target_len {
        colors.truncate(target_len);
        return colors;
    }

    while colors.len() < target_len {
        let old_len = colors.len();

        let mut result = Vec::with_capacity(
            target_len.min(old_len * 2 - 1)
        );

        for i in 0..old_len - 1 {
            result.push(colors[i]);
            result.push(midpoint(colors[i], colors[i + 1]));
        }
        result.push(colors[old_len - 1]);
        colors = result;
    }

    colors.truncate(target_len);
    colors
  } 

  fn midpoint(c1: MyColor,c2: MyColor) -> MyColor{
    MyColor { r: ((c1.r as u16 + c2.r as u16) / 2) as u8, g: ((c1.g as u16 + c2.g as u16) / 2) as u8, b: ((c1.b as u16 + c2.b as u16) / 2) as u8, a: ((c1.a as u16 + c2.a as u16) / 2) as u8}
  }



pub enum CaptureSource {
    DefaultInput,
    PipeWireMonitor,
    NamedDevice(String),
}

impl From<&str> for CaptureSource{
    fn from(value: &str) -> Self {
        match value{
            "Default" => {CaptureSource::DefaultInput},
            "Pipewire" => {CaptureSource::PipeWireMonitor},
            _ => {CaptureSource::NamedDevice(value.to_string())},
        }
    }
}

pub enum ChannelBuff{
}



}