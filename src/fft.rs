pub mod FFT{
    use std::time::Duration;
    use ringbuf::traits::Consumer;
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
        let (mut lframe,mut rframe) = (Vec::<f32>::with_capacity(FRAME_SIZE),Vec::<f32>::with_capacity(FRAME_SIZE));
        while let (Some(lsample),Some(rsample)) = (lcons.try_pop(),rcons.try_pop()){
            lframe.push(lsample);
            rframe.push(rsample);
            if lframe.len() == FRAME_SIZE && lframe.len() == rframe.len(){
                process_frame(&lframe,sample_rate,channels);
                lframe.clear();
                process_frame(&rframe,sample_rate,channels);
                rframe.clear();
            }
        }
        std::thread::sleep(Duration::from_millis(1)); 
    }

    pub fn process_frame(frame: &Vec<f32>,sample_rate: u32,channels: u16){
        let bin_freq = sample_rate as f32 / FFT_SIZE as f32;


    }







}