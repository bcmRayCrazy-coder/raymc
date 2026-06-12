use iced::{Size, Task};

use crate::ui::{
    app::App,
    message::{Message, StateMessage},
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub page_size: Size,
    pub is_playing: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            page_size: Size::new(1024.0, 768.0),
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
            StateMessage::OnPlayStateChanged(is_playing) => {
                self.state.is_playing = is_playing;
                Task::done(Message::UpdatePageState(Box::new(self.state.clone())))
            }
        }
    }
}
