use iced::Task;

use crate::{
    player::playlist::PlaylistTrait,
    ui::{
        app::App,
        message::{
            AudioMessage::{self, UpdatePlayerSong},
            Message, PlayerMessage,
        },
    },
};

impl App {
    pub fn update_player(&mut self, message: PlayerMessage) -> Task<Message> {
        match message {
            PlayerMessage::LoopNext => {
                self.player_manager.loop_next();
                Task::done(Message::Audio(AudioMessage::UpdatePlayerSong))
            }
            PlayerMessage::ListNext => {
                self.player_manager.list_next();
                Task::done(Message::Audio(AudioMessage::UpdatePlayerSong))
            }
            PlayerMessage::ListPrev => {
                self.player_manager.list_prev();
                Task::done(Message::Audio(AudioMessage::UpdatePlayerSong))
            }

            PlayerMessage::InsertJumpNext(new) => {
                let pos = self
                    .player_manager
                    .playlist
                    .insert_next(self.player_manager.current, vec![new]);
                self.player_manager.current = Some(pos);
                Task::done(Message::Audio(UpdatePlayerSong))
            }
        }
    }
}
