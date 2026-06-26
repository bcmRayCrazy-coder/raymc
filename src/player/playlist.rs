use std::{fs, path::PathBuf};

use crate::audio::loader;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PlaylistLoopMode {
    SingleLoop,
    SingleOnce,
    ListOnce,
    ListSorted,
    ListRandom,
}

pub type Playlist = Vec<PathBuf>;

pub trait PlaylistTrait {
    fn get_next_loop(&self, mode: &PlaylistLoopMode, current: Option<usize>) -> Option<usize>;
    fn get_list_next(&self, current: Option<usize>) -> Option<usize>;
    fn get_list_prev(&self, current: Option<usize>) -> Option<usize>;

    fn insert_next(&mut self, current: Option<usize>, new: Vec<PathBuf>) -> usize;

    fn from_song_dir(song_dir: PathBuf) -> Self;
}

impl PlaylistTrait for Playlist {
    fn get_next_loop(&self, mode: &PlaylistLoopMode, current: Option<usize>) -> Option<usize> {
        if self.len() == 0 {
            return None;
        }
        match mode {
            PlaylistLoopMode::SingleOnce | PlaylistLoopMode::SingleLoop => match current {
                Some(c) => Some(c),
                None => Some(0),
            },
            PlaylistLoopMode::ListOnce => {
                if let Some(current) = current {
                    let next = current + 1;
                    if next >= self.len() { None } else { Some(next) }
                } else {
                    Some(0)
                }
            }
            PlaylistLoopMode::ListSorted => {
                if let Some(current) = current {
                    let mut next = current + 1;
                    if next >= self.len() {
                        next = 0;
                    }
                    Some(next)
                } else {
                    Some(0)
                }
            }
            PlaylistLoopMode::ListRandom => Some(rand::random_range(0..=(self.len() - 1))),
        }
    }

    fn get_list_next(&self, current: Option<usize>) -> Option<usize> {
        if self.len() == 0 {
            return None;
        }

        if let Some(current) = current {
            let mut next = current + 1;
            if next >= self.len() {
                next = 0;
            }
            Some(next)
        } else {
            Some(0)
        }
    }

    fn get_list_prev(&self, current: Option<usize>) -> Option<usize> {
        if self.len() == 0 {
            return None;
        }

        if let Some(current) = current {
            if current == 0 {
                Some(self.len() - 1)
            } else {
                Some(current - 1)
            }
        } else {
            Some(0)
        }
    }

    fn insert_next(&mut self, current: Option<usize>, new: Vec<PathBuf>) -> usize {
        match current {
            None => {
                let begin = self.len();
                self.append(&mut new.clone());
                begin
            }
            Some(current) => {
                let mut tail = self.split_off(current + 1);
                self.append(&mut new.clone());
                self.append(&mut tail);
                current + 1
            }
        }
    }

    fn from_song_dir(song_dir: PathBuf) -> Self {
        let mut playlist = Vec::new();

        if let Ok(entries) = fs::read_dir(&song_dir) {
            for entry in entries {
                if let Ok(entry) = entry
                    && let Ok(file_type) = entry.file_type()
                    && file_type.is_file()
                    && let Some(Some(extension)) = entry.path().extension().map(|s| s.to_str())
                    && loader::is_supported_type(&extension)
                    && let Some(file_name) = entry.file_name().to_str()
                {
                    let path = song_dir.clone().join(file_name);
                    // println!("Song Path {:?}", path);
                    playlist.push(path);
                }
            }
        }

        playlist
    }
}
