pub mod Model{

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








}