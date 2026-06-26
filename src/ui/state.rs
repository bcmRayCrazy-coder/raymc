use iced::{Size, Task};

use crate::{
    player::song::PlaySong,
    ui::{
        app::App,
        message::{Message, StateMessage},
    },
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub page_size: Size,
    pub current_song: Option<PlaySong>,
    pub is_playing: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            page_size: Size::new(1024.0, 768.0),
            current_song: None,
            is_playing: false,
        }
    }
}

impl App {
    pub fn update_state(&mut self, message: StateMessage) -> Task<Message> {
        match message {
            StateMessage::Fetch => {
                Task::done(Message::UpdatePageState(Box::new(self.state.clone())))
            }
            StateMessage::OnWindowResize(size) => {
                self.state.page_size = size;
                Task::done(Message::UpdatePageState(Box::new(self.state.clone())))
            }
            StateMessage::OnCurrentSongChange(current_song) => {
                self.state.current_song = current_song;
                Task::done(Message::UpdatePageState(Box::new(self.state.clone())))
            }
            StateMessage::OnPlayStateChange(is_playing) => {
                self.state.is_playing = is_playing;
                Task::done(Message::UpdatePageState(Box::new(self.state.clone())))
            }
        }
    }
}
