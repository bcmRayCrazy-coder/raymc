use crate::player::{
    playlist::{Playlist, PlaylistLoopMode, PlaylistTrait},
    song::PlaySong,
};

pub struct PlayerManager {
    pub playlist: Playlist,
    pub current: Option<usize>,
    pub loop_mode: PlaylistLoopMode,
}

impl PlayerManager {
    pub fn loop_next(&mut self) {
        self.current = self.playlist.get_next_loop(&self.loop_mode, self.current);
    }

    pub fn list_next(&mut self) {
        self.current = self.playlist.get_list_next(self.current);
    }
    pub fn list_prev(&mut self) {
        self.current = self.playlist.get_list_prev(self.current);
    }

    pub fn current(&self) -> Option<usize> {
        self.current.clone()
    }

    pub fn current_song(&self) -> Option<PlaySong> {
        match self.current {
            None => None,
            Some(i) => {
                if i >= self.playlist.len() {
                    None
                } else {
                    Some(self.playlist[i].clone())
                }
            }
        }
    }

    pub fn is_playing(&self) -> bool {
        self.current.is_some()
    }
}

impl Default for PlayerManager {
    fn default() -> Self {
        Self {
            playlist: Vec::new(),
            current: None,
            loop_mode: PlaylistLoopMode::ListSorted,
        }
    }
}
