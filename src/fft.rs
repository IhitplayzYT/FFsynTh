pub mod FFT{
    use std::{f32::consts::PI, sync::{Arc, Mutex}, time::Duration};
    use ringbuf::traits::Consumer;
use rustfft::{Fft, FftPlanner, num_complex::Complex};
    use crate::{FRAME_SIZE, fft, render::render::App};

    const FFT_SIZE: usize = 1024;

    pub fn process_mono_audio(app: Arc<Mutex<App>>, cons: Arc<Mutex<impl Consumer<Item = f32> + Send>>, sample_rate: u32, channels: u16,batch_sz:usize){
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft(FFT_SIZE, rustfft::FftDirection::Forward);
        let mut frame = Vec::<f32>::with_capacity(FRAME_SIZE);
        loop {
            // BATCHING
            for _ in 0..batch_sz{
                let s = cons.lock().unwrap().try_pop();
                if let Some(sample) = s {
                    frame.push(sample);
                } else {
                    break;
                }
            }
            
            if frame.len() >= FFT_SIZE{
                let mut fft_frame: Vec<f32> = frame.drain(..FFT_SIZE).collect();
                Hann_Window_Transform(&mut fft_frame);
                let mut fft_buff:Vec<Complex<f32>> = fft_frame.iter().map(|&x| Complex::new(x, 0.0)).collect();
                process_frame(&mut fft_buff, &fft);

                let magnitudes: Vec<f32> = fft_buff[..FRAME_SIZE/2].iter().map(|x| x.norm()).collect();
                let mut app_guard = app.lock().unwrap();
                let l = app_guard.spectrum.len();
                let r_s = app_guard.rise_speed;
                let f_s = app_guard.fall_speed;
                for (i, &mag) in magnitudes.iter().enumerate() {
                    if i < l{
                        let curr = app_guard.spectrum_peaks[i];
                        let target = mag * app_guard.amp;
                        let new_val = if target > curr {
                            curr * (1.0 - r_s) + target * r_s
                        } else {
                            curr * (1.0 - f_s) + target * f_s 
                        };
                        app_guard.spectrum_peaks[i] = new_val.max(0.0);
                        app_guard.spectrum[i] = new_val.max(0.0);
                    }
                }
            }
        }
    }

    pub fn process_stereo_audio(app: Arc<Mutex<App>>, lcons: Arc<Mutex<impl Consumer<Item = f32> + Send>>, rcons: Arc<Mutex<impl Consumer<Item = f32> + Send>>, sample_rate: u32, channels: u16,batch_sz:usize){
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft(FFT_SIZE, rustfft::FftDirection::Forward);
        let (mut lframe,mut rframe) = (Vec::<f32>::with_capacity(FRAME_SIZE),Vec::<f32>::with_capacity(FRAME_SIZE));    

        loop {
            // Process multiple samples per iteration for better throughput
            for _ in 0..batch_sz {
                let (lsample, rsample) = (lcons.lock().unwrap().try_pop(),rcons.lock().unwrap().try_pop());
                if let (Some(lsample), Some(rsample)) = (lsample, rsample) {
                    lframe.push(lsample);
                    rframe.push(rsample);
                } else {
                    break;
                }
            }
            
            if lframe.len() >= FFT_SIZE && rframe.len() >= FFT_SIZE{
                let mut l_fft_frame: Vec<f32> = lframe.drain(..FFT_SIZE).collect();
                let mut r_fft_frame: Vec<f32> = rframe.drain(..FFT_SIZE).collect();
                
                Hann_Window_Transform(&mut l_fft_frame);
                let mut l_fft_buff:Vec<Complex<f32>> = l_fft_frame.iter().map(|&x| Complex::new(x, 0.0)).collect();
                process_frame(&mut l_fft_buff,&fft);
                let l_mag = l_fft_buff[..FRAME_SIZE/2].iter().map(|x| x.norm()).collect::<Vec<f32>>();

                Hann_Window_Transform(&mut r_fft_frame);
                let mut r_fft_buff:Vec<Complex<f32>> = r_fft_frame.iter().map(|&x| Complex::new(x, 0.0)).collect();
                process_frame(&mut r_fft_buff,&fft);
                let r_mag = r_fft_buff[..FRAME_SIZE/2].iter().map(|x| x.norm()).collect::<Vec<f32>>();


                let mut app_guard = app.lock().unwrap();
                let l = app_guard.spectrum.len();
                let (ll,rl) = (app_guard.left_spectrum.len(),app_guard.right_spectrum.len());
                let r_s = app_guard.rise_speed;
                let f_s = app_guard.fall_speed;
                for (i, (&l_mag_val, &r_mag_val)) in l_mag.iter().zip(r_mag.iter()).enumerate() {
                    if i < ll  && i < rl {

                        let l_curr = app_guard.left_spectrum_peaks[i];
                        let l_tgt = l_mag_val * app_guard.amp;
                        let l_new = if l_tgt > l_curr {
                            (1.0 - r_s) * l_curr + l_tgt * r_s 
                        } else {
                            (1.0 - f_s) * l_curr + l_tgt * f_s 
                        };
                        app_guard.left_spectrum_peaks[i] = l_new.max(0.0);
                        app_guard.left_spectrum[i] = l_new.max(0.0);

                        let r_curr = app_guard.right_spectrum_peaks[i];
                        let r_tgt = r_mag_val * app_guard.amp;
                        let r_new = if r_tgt > r_curr {
                            (1.0 - r_s) * r_curr + r_tgt * r_s 
                        } else {
                            (1.0 - f_s) * r_curr + r_tgt * f_s 
                        };
                        app_guard.right_spectrum_peaks[i] = r_new.max(0.0);
                        app_guard.right_spectrum[i] = r_new.max(0.0);
                        
                        if i < l{
                            let m_curr = app_guard.spectrum_peaks[i];
                            let m_tgt = (l_mag_val + r_mag_val) / 2.0 * app_guard.amp;
                            let m_new = if m_tgt > m_curr {
                                (1.0 - r_s) * m_curr + m_tgt * r_s 
                            } else {
                                (1.0 - f_s) * m_curr + m_tgt * f_s 
                            };
                            app_guard.spectrum_peaks[i] = m_new.max(0.0);
                            app_guard.spectrum[i] = m_new.max(0.0);
                        }
                    }
                }
            }
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