pub mod FFT{
    use std::{f32::consts::PI, sync::Arc, time::Duration};
    use ringbuf::traits::Consumer;
use rustfft::{Fft, FftPlanner, num_complex::Complex};
    use crate::{FRAME_SIZE, fft};

    const FFT_SIZE: usize = 2048;

    pub fn process_mono_audio(cons: &mut impl Consumer<Item = f32>,sample_rate: u32,channels: u16){
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft(FFT_SIZE, rustfft::FftDirection::Forward);
        let mut frame = Vec::<f32>::with_capacity(FRAME_SIZE);
        while let Some(sample) = cons.try_pop(){
            frame.push(sample);
            if frame.len() == FFT_SIZE{
                Hann_Window_Transform(&mut frame);
                let mut fft_buff:Vec<Complex<f32>> = frame.iter().map(|&x| Complex::new(x, 0.0)).collect();
                process_frame(&mut fft_buff, &fft);
                frame.clear();

                // PROCESS
            }
        }
        std::thread::sleep(Duration::from_millis(1)); 
    }

    pub fn process_stereo_audio(lcons: &mut impl Consumer<Item = f32>,rcons: &mut impl Consumer<Item = f32>,sample_rate: u32,channels: u16){
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft(FFT_SIZE, rustfft::FftDirection::Forward);
        let (mut lframe,mut rframe) = (Vec::<f32>::with_capacity(FRAME_SIZE),Vec::<f32>::with_capacity(FRAME_SIZE));
    
        while let (Some(lsample),Some(rsample)) = (lcons.try_pop(),rcons.try_pop()){
            lframe.push(lsample);
            rframe.push(rsample);
            if lframe.len() == FFT_SIZE && lframe.len() == rframe.len(){
                Hann_Window_Transform(&mut lframe);
                let mut l_fft_buff:Vec<Complex<f32>> = lframe.iter().map(|&x| Complex::new(x, 0.0)).collect();
                process_frame(&mut l_fft_buff,&fft);
                lframe.clear();

                // PROCESS
                let l_mag = l_fft_buff[..FRAME_SIZE/2].iter().map(|x| x.norm()).collect::<Vec<f32>>();

                Hann_Window_Transform(&mut rframe);
                let mut r_fft_buff:Vec<Complex<f32>> = rframe.iter().map(|&x| Complex::new(x, 0.0)).collect();
                process_frame(&mut r_fft_buff,&fft);
                rframe.clear();

                // PROCESS
                let r_mag = r_fft_buff[..FRAME_SIZE/2].iter().map(|x| x.norm()).collect::<Vec<f32>>();       
                println!("{} {}",l_mag.iter().any(|x| x != &0.0),r_mag.iter().any(|x| x != &0.0));
            }
        }
        std::thread::sleep(Duration::from_millis(1)); 
    }

    pub fn process_frame(fft_buff: &mut Vec<Complex<f32>>,fft: &Arc<dyn Fft<f32>>){
        fft.process(fft_buff);
    }

    pub fn Hann_Window_Transform(frame: &mut Vec<f32>){
        for i in 0..FFT_SIZE{
            let window = 0.5 * (1.0 - ((2.0 * PI * i as f32)/(FFT_SIZE-1) as f32).cos());
            frame[i] *= window;
        }
    }







}