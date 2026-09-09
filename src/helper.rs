pub mod Helper{
    use std::process::exit;

use crate::model::Model::MyColor;



    const DBG_STR: &str = "";
    const OK:i32 = 0;
    const ERR:i32 = -1;


    #[derive(Debug,Clone)]
    pub struct CLI{
        pub dbg: bool,
        pub bars: usize,
        pub src: Option<String>,
        pub amp: f64,
        pub colors: Vec<MyColor>
    }


    pub fn Help(){
        println!("{DBG_STR}");
        exit(OK);
    }


    impl CLI{
        pub fn new() -> Self{
            Self {dbg: false,bars: 5,src:None,amp:1.0,colors:vec![]}
        }

        pub fn Parse_Args(&mut self){

            let mut args: Vec<String> = std::env::args().collect();
            if args.len() > 0{
                args = args[1..].to_vec();
            }
            for i in &args{
                if i == "-d" || i == "--debug" || i == " --DEBUG" || i == "-D"{
                    self.dbg = true;
                } else if i == "-h" || i == "--help" || i == " --HELP" || i == "-H"{
                    Help();
                } else if i.starts_with("-n=") || i.starts_with("--n_bars="){
                    self.bars = i[i.find("=").unwrap()+1..].parse().expect("Bar Count is an Unsigned Int");
                } else if i.starts_with("-amp=") || i.starts_with("--amplification="){
                    self.amp = i[i.find("=").unwrap()+1..].parse().expect("Bar Count is an Unsigned Int");
                } else if i.starts_with("-src=") || i.starts_with("--src_audio="){
                    self.src = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--color=") || i.starts_with("-c="){
                    self.colors.push(MyColor::from(&i[i.find("=").unwrap()+1..]));
                } else{
                    Help();
                }
            } 
        }



    }


    





}