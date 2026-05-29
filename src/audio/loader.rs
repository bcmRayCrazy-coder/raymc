use std::io::Seek;

// use audrey::{Reader, read::ReadError};
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

static DIV_128: f32 = 1.0 / 128.0;

fn convert_audio_buffer(gernic_buf: GenericAudioBufferRef<'_>) -> Option<Vec<[f32; 2]>> {
    let channels = gernic_buf.spec().channels().count();
    let frames = gernic_buf.frames();

    match gernic_buf {
        GenericAudioBufferRef::F32(buf) => {
            if channels == 1 {
                let plane = buf.plane(0)?;
                Some((0..frames).map(|i| [plane[i], plane[i]]).collect())
            } else {
                let left = buf.plane(0)?;
                let right = buf.plane(1)?;
                Some((0..frames).map(|i| [left[i], right[i]]).collect())
            }
        }
        GenericAudioBufferRef::U8(buf) => {
            if channels == 1 {
                let plane = buf.plane(0)?;
                Some(
                    (0..frames)
                        .map(|i| {
                            let s = (plane[i] as f32 - 128.0) * DIV_128;
                            [s, s]
                        })
                        .collect(),
                )
            } else {
                let left = buf.plane(0)?;
                let right = buf.plane(1)?;
                Some(
                    (0..frames)
                        .map(|i| {
                            [
                                (left[i] as f32 - 128.0) * DIV_128,
                                (right[i] as f32 - 128.0) * DIV_128,
                            ]
                        })
                        .collect(),
                )
            }
        }

        GenericAudioBufferRef::S16(buf) => {
            let scale = 1.0 / i16::MAX as f32;
            if channels == 1 {
                let plane = buf.plane(0)?;
                Some(
                    (0..frames)
                        .map(|i| {
                            let s = (plane[i] as f32 * scale);
                            [s, s]
                        })
                        .collect(),
                )
            } else {
                let left = buf.plane(0)?;
                let right = buf.plane(1)?;
                Some(
                    (0..frames)
                        .map(|i| [left[i] as f32 * scale, right[i] as f32 * scale])
                        .collect(),
                )
            }
        }
        // TODO: Finish them (💀)
        GenericAudioBufferRef::U16(_buf) => todo!("u16"),
        GenericAudioBufferRef::U24(_buf) => todo!("u24"),
        GenericAudioBufferRef::U32(_buf) => todo!("u32"),
        GenericAudioBufferRef::S8(_buf) => todo!("s8"),
        GenericAudioBufferRef::S24(_buf) => todo!("s24"),
        GenericAudioBufferRef::S32(_buf) => todo!("s32"),
        GenericAudioBufferRef::F64(_buf) => todo!("f64"),
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
