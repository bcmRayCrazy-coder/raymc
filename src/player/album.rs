use std::{fmt::Display, fs, path::PathBuf};

use crate::audio::loader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlbumName {
    Single,
    Album(String),
}

impl AlbumName {
    pub fn get_dir(&self, album_dir: PathBuf) -> PathBuf {
        match self {
            AlbumName::Single => album_dir,
            AlbumName::Album(dir) => album_dir.join(dir),
        }
    }

    pub fn get_songs(&self, album_dir: PathBuf) -> Vec<String> {
        let mut song_list = Vec::new();
        if let Ok(entries) = fs::read_dir(self.get_dir(album_dir)) {
            for entry in entries {
                if let Ok(entry) = entry
                    && let Ok(file_type) = entry.file_type()
                    && file_type.is_file()
                    && let Some(Some(extension)) = entry.path().extension().map(|s| s.to_str())
                    && loader::is_supported_type(&extension)
                    && let Some(file_name) = entry.file_name().to_str()
                {
                    song_list.push(file_name.to_owned());
                }
            }
        }

        song_list
    }
}

impl Display for AlbumName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AlbumName::Single => "Single",
                AlbumName::Album(name) => name,
            }
        )
    }
}

pub fn get_album_list(album_dir: &PathBuf) -> Vec<AlbumName> {
    let mut album_list = Vec::new();
    if let Ok(entries) = fs::read_dir(album_dir) {
        for entry in entries {
            if let Ok(entry) = entry
                && let Ok(file_type) = entry.file_type()
                && file_type.is_dir()
                && let Some(name) = entry.file_name().to_str()
            {
                album_list.push(AlbumName::Album(name.to_owned()));
            }
        }
    }

    album_list
}
