use std::path::PathBuf;

use crate::player::r#loop::PlayerLoopMode;

pub struct PlayerManager {
    pub playlist: Vec<PathBuf>,
    pub current: Option<usize>,
    pub loop_mode: PlayerLoopMode,
}

impl PlayerManager {
    pub fn loop_next(&mut self) {
        if self.playlist.len() == 0 {
            self.current = None;
            return;
        }
        match self.loop_mode {
            PlayerLoopMode::SingleOnce | PlayerLoopMode::SingleLoop => {
                self.current = match self.current {
                    Some(c) => Some(c),
                    None => Some(0),
                }
            }
            PlayerLoopMode::ListOnce => {
                if let Some(current) = self.current {
                    let next = current + 1;
                    if next >= self.playlist.len() {
                        self.current = None;
                    } else {
                        self.current = Some(next);
                    }
                } else {
                    self.current = Some(0);
                }
            }
            PlayerLoopMode::ListSorted => {
                if let Some(current) = self.current {
                    let mut next = current + 1;
                    if next >= self.playlist.len() {
                        next = 0;
                    }
                    self.current = Some(next);
                } else {
                    self.current = Some(0);
                }
            }
            PlayerLoopMode::ListRandom => {
                self.current = Some(rand::random_range(0..=(self.playlist.len() - 1)));
            }
        }
    }

    pub fn list_next(&mut self) {
        if self.playlist.len() == 0 {
            self.current = None;
            return;
        }

        if let Some(current) = self.current {
            let mut next = current + 1;
            if next >= self.playlist.len() {
                next = 0;
            }
            self.current = Some(next);
        } else {
            self.current = Some(0);
        }
    }
    pub fn list_prev(&mut self) {
        if self.playlist.len() == 0 {
            self.current = None;
            return;
        }

        if let Some(current) = self.current {
            if current == 0 {
                self.current = Some(self.playlist.len() - 1);
            } else {
                self.current = Some(current - 1);
            }
        } else {
            self.current = Some(0);
        }
    }
}

impl Default for PlayerManager {
    fn default() -> Self {
        Self {
            playlist: Vec::new(),
            current: None,
            loop_mode: PlayerLoopMode::ListSorted,
        }
    }
}
