use std::fmt::Debug;

use cpal::{
    Device, FromSample, OutputCallbackInfo, SampleFormat, SizedSample, Stream,
    SupportedStreamConfig,
    traits::{DeviceTrait, StreamTrait},
};

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum AudioError {
    BuildStreamError(cpal::Error),
    UnsupportFormat(SampleFormat),
    PlayStreamError(cpal::Error),
    PauseStreamError(cpal::Error),
    StreamNotBuild,
}

pub struct AudioStream {
    device: Device,
    config: SupportedStreamConfig,
    stream: Option<Stream>,
}

impl AudioStream {
    pub fn new(device: Device, config: SupportedStreamConfig) -> Self {
        Self {
            device,
            config,
            stream: None,
        }
    }

    pub fn build_stream<T, D>(&mut self, data_callback: D) -> Result<(), AudioError>
    where
        T: FromSample<f32> + SizedSample,
        D: FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static,
    {
        let config = self.config.config();
        let stream = self.device.build_output_stream(
            config,
            data_callback,
            |err| eprintln!("Error building output stream {}", err),
            None,
        );
        match stream {
            Ok(s) => {
                self.stream = Some(s);
                Ok(())
            }
            Err(e) => Err(AudioError::BuildStreamError(e)),
        }
    }

    pub fn channels(&self) -> u16 {
        self.config.channels()
    }

    pub fn play_stream(&self) -> Result<(), AudioError> {
        if let Some(stream) = &self.stream {
            return stream
                .play()
                .map_err(|err| AudioError::PlayStreamError(err));
        }
        Err(AudioError::StreamNotBuild)
    }

    pub fn pause_stream(&self) -> Result<(), AudioError> {
        if let Some(stream) = &self.stream {
            return stream
                .pause()
                .map_err(|err| AudioError::PauseStreamError(err));
        }
        Err(AudioError::StreamNotBuild)
    }

    pub fn sample_rate(&self) -> u32 {
        self.config.sample_rate()
    }

    pub fn sample_format(&self) -> SampleFormat {
        self.config.sample_format()
    }
}

impl Drop for AudioStream {
    fn drop(&mut self) {
        let _ = self.pause_stream();
    }
}
