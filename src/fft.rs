pub mod FFT{
    use std::{f32::consts::PI, time::Duration};
    use ringbuf::traits::Consumer;
use rustfft::{FftPlanner, num_complex::Complex};
    use crate::FRAME_SIZE;

    const FFT_SIZE: usize = 2048;

    pub fn process_mono_audio(cons: &mut impl Consumer<Item = f32>,sample_rate: u32,channels: u16){
        let mut frame = Vec::<f32>::with_capacity(FRAME_SIZE);
        while let Some(sample) = cons.try_pop(){
            frame.push(sample);
            if frame.len() == FRAME_SIZE{
                process_frame(&frame,sample_rate,channels);
                frame.clear();
            }
        }
        std::thread::sleep(Duration::from_millis(1)); 
    }

    pub fn process_stereo_audio(lcons: &mut impl Consumer<Item = f32>,rcons: &mut impl Consumer<Item = f32>,sample_rate: u32,channels: u16){
        let mut planner = FftPlanner::<f32>::new();
        let (mut lframe,mut rframe) = (Vec::<f32>::with_capacity(FRAME_SIZE),Vec::<f32>::with_capacity(FRAME_SIZE));
        while let (Some(lsample),Some(rsample)) = (lcons.try_pop(),rcons.try_pop()){
            lframe.push(lsample);
            rframe.push(rsample);
            if lframe.len() == FRAME_SIZE && lframe.len() == rframe.len(){
                process_frame(&mut lframe,sample_rate,channels,&mut planner);
                lframe.clear();
                process_frame(&mut rframe,sample_rate,channels,&mut planner);
                rframe.clear();
            }
        }
        std::thread::sleep(Duration::from_millis(1)); 
    }

    pub fn process_frame(frame: &mut Vec<f32>,sample_rate: u32,channels: u16,fftplanner: &mut FftPlanner<f32>){
        let bin_freq = sample_rate as f32 / FFT_SIZE as f32;

        // Hann Window (Prevents signal leakage between frames)
        for i in 0..FFT_SIZE{
            let window = 0.5 * (1.0 - ((2.0 * PI * i as f32)/(FFT_SIZE-1) as f32).cos());
            frame[i] *= window;
        }

        let fft = fftplanner.plan_fft(FFT_SIZE, rustfft::FftDirection::Forward);
        let mut fft_buff:Vec<Complex<f32>> = frame.iter().map(|&x| Complex::new(x, 0.0)).collect();
        fft.process(&mut fft_buff);
    }







}