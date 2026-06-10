use iced::Task;

use crate::ui::{
    app::App,
    message::{Message, PlayerMessage},
};

impl App {
    pub fn update_player(&mut self, message: PlayerMessage) -> Task<Message> {
        match message {
            PlayerMessage::LoopNext => {
                self.player_manager.loop_next();
                Task::none()
            }
            PlayerMessage::ListNext => {
                self.player_manager.list_next();
                Task::none()
            }
            PlayerMessage::ListPrev => {
                self.player_manager.list_prev();
                Task::none()
            }
        }
    }
}
