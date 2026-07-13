use iced::Task;

use crate::{
    media_control::manager::MediaControlManager,
    ui::{
        app::App,
        message::{MediaControlMessage, Message},
    },
};

impl App {
    pub fn update_media_control(&mut self, message: MediaControlMessage) -> Task<Message> {
        match message {
            MediaControlMessage::UpdateSong(song) => match self.media_control_manager.as_mut() {
                Some(media_control_manager) => {
                    match song {
                        Some(song) => {
                            let _ = media_control_manager
                                .attach()
                                .inspect_err(|err| eprintln!("Media Control Error\n{:?}", err));
                            let _ = media_control_manager
                                .set_song(song)
                                .inspect_err(|err| eprintln!("Media Control Error\n{:?}", err));
                        }
                        None => {
                            let _ = media_control_manager
                                .detach()
                                .inspect_err(|err| eprintln!("Media Control Error\n{:?}", err));
                        }
                    };
                    Task::none()
                }
                None => Task::done(Message::MediaControl(MediaControlMessage::Init)),
            },

            MediaControlMessage::Init => {
                if self.media_control_manager.is_none() {
                    match MediaControlManager::new() {
                        Ok(media_control_manager) => {
                            self.media_control_manager = Some(media_control_manager)
                        }
                        Err(err) => eprintln!("Unable to init Media Control Manager\n{:?}", err),
                    }
                }
                Task::none()
            }
        }
    }
}
