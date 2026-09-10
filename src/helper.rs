pub mod Helper{
    use std::process::exit;

use crate::model::Model::MyColor;



    const DBG_STR: &str = r#"
FFsynTh - TUI Audio Spectrum Visualizer

USAGE:
    ffsynth [OPTIONS]

OPTIONS:
    -h, --help              Show this help message and exit
    -d, --debug             Enable debug mode (print CLI arguments)
    -n, --n_bars=<NUM>      Set number of bars in visualization (default: 5)
    -amp, --amplification=<NUM>  Set amplification factor (default: 1.0)
    -src, --src_audio=<PATH>    Set audio source device (default: pipewire)
    -c, --color=<R,G,B,A>   Add a color for gradient (format: R,G,B or R,G,B,A)
                            Example: -c=255,0,0 or -c=0,255,0,128

EXAMPLES:
    ffsynth                              # Run with default settings
    ffsynth -d -n=20                     # Debug mode with 20 bars
    ffsynth -c=255,0,0 -c=0,0,255       # Red to blue gradient
    ffsynth -amp=1.5 -n=30              # Higher amplification with 30 bars
    ffsynth -src=default                # Use default audio device

NOTES:
    - Colors are specified as RGB or RGBA values (0-255)
    - If 1 color is provided, it fades to transparent
    - If 2+ colors are provided, they create a gradient
    - The visualizer automatically interpolates between colors
"#;
    const OK:i32 = 0;
    const ERR:i32 = -1;


    #[derive(Debug,Clone)]
    pub struct CLI{
        pub dbg: bool,
        pub bars: usize,
        pub src: Option<String>,
        pub amp: f64,
        pub colors: Vec<MyColor>,
        pub stereo: bool
    }


    pub fn Help(){
        println!("{DBG_STR}");
        exit(OK);
    }


    impl CLI{
        pub fn new() -> Self{
            Self {dbg: false,bars: 5,src:None,amp:1.0,colors:vec![],stereo: true}
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
                } else if i == "--mono" || i == "-m"{
                    self.stereo = false;
                } else if i == "--stereo" || i == "-s"{
                    self.stereo = true;
                } else{
                    Help();
                }
            } 
        }



    }


    





}