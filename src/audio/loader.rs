use std::io::Seek;

use symphonia::core::{
    audio::{Audio, GenericAudioBufferRef},
    codecs::audio::AudioDecoderOptions,
    formats::{FormatOptions, TrackType, probe::Hint},
    io::{MediaSource, MediaSourceStream},
    meta::MetadataOptions,
};

use crate::{
    audio::track::{AudioTrack, AudioTrackType},
    embed::get_embed_file,
};

#[derive(Debug)]
pub enum AudioLoaderError {
    UnsupportedFormat,
    FileNotFound,
    NoAudio,
}

const SUPPORTED_TYPE_EXT: &[&str] = &["mp3", "wav"];
pub fn is_supported_type(ext: &str) -> bool {
    SUPPORTED_TYPE_EXT.contains(&ext.to_lowercase().as_str())
}

macro_rules! convert_audio {
    ($buf:expr, $channels:expr, $frames:expr, $scale:expr, $offset:expr) => {
        if $channels == 1 {
            let plane = $buf.plane(0)?;
            Some(
                (0..$frames)
                    .map(|i| {
                        let s = (plane[i] as f32 - $offset) * $scale;
                        [s, s]
                    })
                    .collect(),
            )
        } else {
            let left = $buf.plane(0)?;
            let right = $buf.plane(1)?;
            Some(
                (0..$frames)
                    .map(|i| {
                        [
                            (left[i] as f32 - $offset) * $scale,
                            (right[i] as f32 - $offset) * $scale,
                        ]
                    })
                    .collect(),
            )
        }
    };
}

fn convert_audio_buffer(gernic_buf: GenericAudioBufferRef<'_>) -> Option<Vec<[f32; 2]>> {
    let channels = gernic_buf.spec().channels().count();
    let frames = gernic_buf.frames();

    match gernic_buf {
        GenericAudioBufferRef::F32(buf) => convert_audio!(buf, channels, frames, 1.0, 0.0),
        GenericAudioBufferRef::F64(buf) => convert_audio!(buf, channels, frames, 1.0, 0.0),
        GenericAudioBufferRef::S8(buf) => convert_audio!(buf, channels, frames, 1.0 / 127.0, 0.0),
        GenericAudioBufferRef::S16(buf) => {
            convert_audio!(buf, channels, frames, 1.0 / 32767.0, 0.0)
        }
        GenericAudioBufferRef::S32(buf) => {
            convert_audio!(buf, channels, frames, 1.0 / 2147483647.0, 0.0)
        }
        GenericAudioBufferRef::U8(buf) => convert_audio!(buf, channels, frames, 1.0 / 128.0, 128.0),
        GenericAudioBufferRef::U16(buf) => {
            convert_audio!(buf, channels, frames, 1.0 / 32768.0, 32768.0)
        }
        GenericAudioBufferRef::U32(buf) => {
            convert_audio!(buf, channels, frames, 1.0 / 2147483648.0, 2147483648.0)
        }
        _ => None,
    }
}

impl AudioTrack {
    pub fn from_reader<T>(reader: T, track_type: AudioTrackType) -> Result<Self, AudioLoaderError>
    where
        T: MediaSource + Seek + Send + Sync,
    {
        let hint = Hint::new();
        let mss = MediaSourceStream::new(Box::new(reader), Default::default());

        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();
        let decoder_opts = AudioDecoderOptions::default();

        let mut format_reader = symphonia::default::get_probe()
            .probe(&hint, mss, format_opts, metadata_opts)
            .map_err(|_| AudioLoaderError::UnsupportedFormat)?;

        let track = format_reader
            .default_track(TrackType::Audio)
            .ok_or(AudioLoaderError::NoAudio)?;
        let track_id = track.id;

        let track_params = track
            .codec_params
            .as_ref()
            .ok_or(AudioLoaderError::NoAudio)?
            .audio()
            .ok_or(AudioLoaderError::NoAudio)?;
        let sample_rate = track_params
            .sample_rate
            .ok_or(AudioLoaderError::UnsupportedFormat)?;

        let mut decoder = symphonia::default::get_codecs()
            .make_audio_decoder(track_params, &decoder_opts)
            .map_err(|_| AudioLoaderError::UnsupportedFormat)?;

        let mut raw_samples = Vec::new();

        while let Ok(Some(packet)) = format_reader.next_packet() {
            if packet.track_id != track_id {
                continue;
            }
            match decoder.decode(&packet) {
                Ok(buf) => {
                    raw_samples.extend(
                        convert_audio_buffer(buf).ok_or(AudioLoaderError::UnsupportedFormat)?,
                    );
                }
                Err(err) => {
                    eprintln!("Audio decode error {}! Skip the packet.", err);
                    continue;
                }
            }
        }

        Ok(Self::new(track_type, raw_samples, sample_rate))
    }

    pub fn from_disk(
        file_path: &str,
        track_type: AudioTrackType,
    ) -> Result<AudioTrack, AudioLoaderError> {
        let file = std::fs::File::open(file_path).map_err(|_| AudioLoaderError::FileNotFound)?;
        Self::from_reader(file, track_type)
    }

    pub fn from_embed(
        file_path: &str,
        track_type: AudioTrackType,
    ) -> Result<AudioTrack, AudioLoaderError> {
        let file = get_embed_file(file_path).ok_or(AudioLoaderError::FileNotFound)?;
        Self::from_reader(std::io::Cursor::new(file.data), track_type)
    }
}
