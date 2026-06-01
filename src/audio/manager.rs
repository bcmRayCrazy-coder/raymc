use std::sync::{Arc, Mutex};

use cpal::{
    FromSample, OutputCallbackInfo, SizedSample,
    traits::{DeviceTrait, HostTrait},
};

use crate::audio::{
    mixer::AudioMixer,
    stream::{AudioError, AudioStream},
};

pub struct AudioManager {
    pub stream: Arc<Mutex<AudioStream>>,
    pub mixer: Arc<Mutex<AudioMixer>>,
    pub volume: f32,
    is_started: bool,
}

impl AudioManager {
    pub fn new(stream: AudioStream) -> Self {
        Self {
            mixer: Arc::new(Mutex::new(AudioMixer::new(stream.sample_rate()))),
            stream: Arc::new(Mutex::new(stream)),
            volume: 1.0,
            is_started: false,
        }
    }

    pub fn build_stream(&mut self) -> Result<(), AudioError> {
        let stream = self.stream.lock().unwrap();
        let format = stream.sample_format().clone();
        drop(stream);

        match format {
            cpal::SampleFormat::I8 => self.build_stream_typed::<i8>(),
            cpal::SampleFormat::I16 => self.build_stream_typed::<i16>(),
            cpal::SampleFormat::I32 => self.build_stream_typed::<i32>(),
            cpal::SampleFormat::I64 => self.build_stream_typed::<i64>(),
            cpal::SampleFormat::U8 => self.build_stream_typed::<u8>(),
            cpal::SampleFormat::U16 => self.build_stream_typed::<u16>(),
            cpal::SampleFormat::U32 => self.build_stream_typed::<u32>(),
            cpal::SampleFormat::U64 => self.build_stream_typed::<u64>(),
            cpal::SampleFormat::F32 => self.build_stream_typed::<f32>(),
            cpal::SampleFormat::F64 => self.build_stream_typed::<f64>(),
            format => Err(AudioError::UnsupportFormat(format)),
        }
    }

    pub fn build_stream_typed<T>(&mut self) -> Result<(), AudioError>
    where
        T: FromSample<f32> + SizedSample,
    {
        if !self.is_started() {
            let mut stream = self.stream.lock().unwrap();
            let channel_num = stream.channels() as usize;
            let mixer = self.mixer.clone();

            self.is_started = true;

            return stream.build_stream(move |output: &mut [T], _: &OutputCallbackInfo| {
                for frame in output.chunks_mut(channel_num) {
                    let mut mixer = mixer.lock().unwrap();
                    let sample = mixer.tick_sample();
                    for (channel, output_sample) in frame.iter_mut().enumerate() {
                        *output_sample = T::from_sample(match channel {
                            0 => sample[0],
                            1 => sample[1],
                            _ => 0.0,
                        })
                    }
                }
            });
        }
        Ok(())
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Unable to get default audio device");
        let config = device
            .default_output_config()
            .expect("Unable to get default audio config");

        let stream = AudioStream::new(host, device, config);
        let mixer = AudioMixer::new(stream.sample_rate());

        Self {
            stream: Arc::new(Mutex::new(stream)),
            mixer: Arc::new(Mutex::new(mixer)),
            volume: 1.0,
            is_started: false,
        }
    }
}
