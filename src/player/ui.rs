use iced::Task;

use crate::{
    player::playlist::{Playlist, PlaylistTrait},
    ui::{
        app::App,
        message::{AudioMessage::UpdatePlayerSong, Message, PlayerMessage, StateMessage},
    },
};

impl App {
    pub fn update_player(&mut self, message: PlayerMessage) -> Task<Message> {
        match message {
            PlayerMessage::LoopNext => {
                self.player_manager.loop_next();
                Task::batch([
                    Task::done(Message::State(StateMessage::OnPlayStateChanged(
                        self.player_manager.is_playing(),
                    ))),
                    Task::done(Message::Audio(UpdatePlayerSong)),
                ])
            }
            PlayerMessage::ListNext => {
                self.player_manager.list_next();
                Task::batch([
                    Task::done(Message::State(StateMessage::OnPlayStateChanged(
                        self.player_manager.is_playing(),
                    ))),
                    Task::done(Message::Audio(UpdatePlayerSong)),
                ])
            }
            PlayerMessage::ListPrev => {
                self.player_manager.list_prev();
                Task::batch([
                    Task::done(Message::State(StateMessage::OnPlayStateChanged(
                        self.player_manager.is_playing(),
                    ))),
                    Task::done(Message::Audio(UpdatePlayerSong)),
                ])
            }
            PlayerMessage::PlayEnd => Task::batch([
                Task::done(Message::State(StateMessage::OnPlayStateChanged(false))),
                Task::done(Message::Player(PlayerMessage::LoopNext)),
            ]),

            PlayerMessage::InsertJumpNext(new) => {
                let pos = self
                    .player_manager
                    .playlist
                    .insert_next(self.player_manager.current, vec![new]);
                self.player_manager.current = Some(pos);
                Task::batch([
                    Task::done(Message::State(StateMessage::OnPlayStateChanged(
                        self.player_manager.is_playing(),
                    ))),
                    Task::done(Message::Audio(UpdatePlayerSong)),
                ])
            }

            PlayerMessage::InsertJumpNextAlbum(album, dir) => {
                let pos = self.player_manager.playlist.len();
                self.player_manager
                    .playlist
                    .append(&mut Playlist::from_song_dir(album.get_dir(dir)));

                self.player_manager.current = Some(pos);
                Task::batch([
                    Task::done(Message::State(StateMessage::OnPlayStateChanged(
                        self.player_manager.is_playing(),
                    ))),
                    Task::done(Message::Audio(UpdatePlayerSong)),
                ])
            }
        }
    }
}
