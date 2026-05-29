use std::{
    io::{Read, Seek},
    path::Path,
};

use audrey::{Reader, read::ReadError};

use crate::{
    audio::track::{AudioTrack, AudioTrackType},
    embed::get_embed_file,
};

#[derive(Debug)]
pub enum AudioLoaderError {
    ReadError(ReadError),
    UnsupportType,
    FileNotFound,
}

pub fn is_supported_type(path: &Path) -> bool {
    path.extension()
        .map(|ext| {
            ext.eq_ignore_ascii_case("flac")
                || ext.eq_ignore_ascii_case("ogg")
                || ext.eq_ignore_ascii_case("wav")
        })
        .unwrap_or(false)
}

impl AudioTrack {
    pub fn from_reader<T>(
        raw_sample: &mut Reader<T>,
        play_type: AudioTrackType,
    ) -> Result<AudioTrack, AudioLoaderError>
    where
        T: Read + Seek,
    {
        let desc = raw_sample.description();
        let channel_count = desc.channel_count();

        let sample: Vec<[f32; 2]> = match channel_count {
            1 => raw_sample
                .samples()
                .filter_map(|s| s.ok())
                .map(|s| [s, s])
                .collect(),
            2 => raw_sample
                .samples()
                .filter_map(|s| s.ok())
                .collect::<Vec<f32>>()
                .chunks_exact(2)
                .map(|chunk| [chunk[0], chunk[1]])
                .collect(),
            _ => vec![[0.0, 0.0]],
        };

        Ok(AudioTrack::new(
            play_type,
            sample.clone(),
            desc.sample_rate(),
        ))
    }

    pub fn from_disk(
        file_path: &str,
        play_type: AudioTrackType,
    ) -> Result<AudioTrack, AudioLoaderError> {
        let mut raw_sample = audrey::open(file_path).map_err(|e| AudioLoaderError::ReadError(e))?;
        Self::from_reader(&mut raw_sample, play_type)
    }

    pub fn from_embed(
        file_path: &str,
        play_type: AudioTrackType,
    ) -> Result<AudioTrack, AudioLoaderError> {
        let file = get_embed_file(file_path).ok_or(AudioLoaderError::FileNotFound)?;
        let mut raw_sample = audrey::read::Reader::new(std::io::Cursor::new(file.data))
            .map_err(|err| AudioLoaderError::ReadError(err))?;
        Self::from_reader(&mut raw_sample, play_type)
    }
}
