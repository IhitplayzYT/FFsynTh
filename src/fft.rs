pub mod FFT{
    use std::{f32::consts::PI, sync::{Arc, Mutex}, time::Duration};
    use ringbuf::traits::Consumer;
use rustfft::{Fft, FftPlanner, num_complex::Complex};
    use crate::{FRAME_SIZE, fft, render::render::App};

    const FFT_SIZE: usize = 2048;

    pub fn process_mono_audio(app: Arc<Mutex<App>>, cons: Arc<Mutex<impl Consumer<Item = f32> + Send>>, sample_rate: u32, channels: u16){
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft(FFT_SIZE, rustfft::FftDirection::Forward);
        let mut frame = Vec::<f32>::with_capacity(FRAME_SIZE);
        loop {
            let s = cons.lock().unwrap().try_pop();

            if let Some(sample) = s {
                frame.push(sample);
                if frame.len() == FFT_SIZE{
                    Hann_Window_Transform(&mut frame);
                    let mut fft_buff:Vec<Complex<f32>> = frame.iter().map(|&x| Complex::new(x, 0.0)).collect();
                    process_frame(&mut fft_buff, &fft);
                    frame.clear();

                    // PROCESS - write to app
                    let magnitudes: Vec<f32> = fft_buff[..FRAME_SIZE/2].iter().map(|x| x.norm()).collect();
                    let mut app_guard = app.lock().unwrap();
                    let l = app_guard.spectrum.len();
                    for (i, &mag) in magnitudes.iter().enumerate() {
                        if i < l{
                            app_guard.spectrum[i] = mag;
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(1)); 
        }
    }

    pub fn process_stereo_audio(app: Arc<Mutex<App>>, lcons: Arc<Mutex<impl Consumer<Item = f32> + Send>>, rcons: Arc<Mutex<impl Consumer<Item = f32> + Send>>, sample_rate: u32, channels: u16){
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft(FFT_SIZE, rustfft::FftDirection::Forward);
        let (mut lframe,mut rframe) = (Vec::<f32>::with_capacity(FRAME_SIZE),Vec::<f32>::with_capacity(FRAME_SIZE));    

        loop {
            let (lsample, rsample) = (lcons.lock().unwrap().try_pop(),rcons.lock().unwrap().try_pop());
            if let (Some(lsample), Some(rsample)) = (lsample, rsample) {
                lframe.push(lsample);
                rframe.push(rsample);
                if lframe.len() == FFT_SIZE && lframe.len() == rframe.len(){
                    Hann_Window_Transform(&mut lframe);
                    let mut l_fft_buff:Vec<Complex<f32>> = lframe.iter().map(|&x| Complex::new(x, 0.0)).collect();
                    process_frame(&mut l_fft_buff,&fft);
                    lframe.clear();
                    let l_mag = l_fft_buff[..FRAME_SIZE/2].iter().map(|x| x.norm()).collect::<Vec<f32>>();

                    Hann_Window_Transform(&mut rframe);
                    let mut r_fft_buff:Vec<Complex<f32>> = rframe.iter().map(|&x| Complex::new(x, 0.0)).collect();
                    process_frame(&mut r_fft_buff,&fft);
                    rframe.clear();
                    let r_mag = r_fft_buff[..FRAME_SIZE/2].iter().map(|x| x.norm()).collect::<Vec<f32>>();

                    // Write to app
                    let mut app_guard = app.lock().unwrap();
                    let l = app_guard.spectrum.len();
                    let (ll,rl) = (app_guard.left_spectrum.len(),app_guard.right_spectrum.len());
                    for (i, (&l_mag_val, &r_mag_val)) in l_mag.iter().zip(r_mag.iter()).enumerate() {
                        if i < ll  && i < rl {
                            app_guard.left_spectrum[i] = l_mag_val;
                            app_guard.right_spectrum[i] = r_mag_val;
                            if i < l{
                                app_guard.spectrum[i] = (l_mag_val + r_mag_val) / 2.0;
                            }
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(1)); 
        }
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