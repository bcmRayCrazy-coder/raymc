use crate::player::playlist::{Playlist, PlaylistLoopMode, PlaylistTrait};

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
