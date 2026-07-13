use iced::{
    Subscription, Task,
    futures::{SinkExt, StreamExt, channel::mpsc},
    stream,
};

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
                None => Task::none(),
            },
            MediaControlMessage::UpdatePlaying(is_playing) => {
                match self.media_control_manager.as_mut() {
                    Some(media_control_manager) => {
                        let _ = media_control_manager
                            .set_playing(is_playing)
                            .inspect_err(|err| eprintln!("Media Control Error\n{:?}", err));
                        Task::none()
                    }
                    None => Task::none(),
                }
            }

            MediaControlMessage::Init(sender) => {
                if self.media_control_manager.is_none() {
                    match MediaControlManager::new(sender) {
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

    pub fn subscription_media_control(&self) -> Subscription<Message> {
        Subscription::run(|| {
            stream::channel(127, async |mut output| {
                let (sender, mut receiver) = mpsc::channel(127);
                output
                    .send(Message::MediaControl(MediaControlMessage::Init(sender)))
                    .await
                    .expect("Unable to open mpsc channel for media control");

                loop {
                    if let Some(input) = receiver.next().await
                        && let Err(err) = output.send(input).await
                    {
                        eprintln!("Failed to send message from media control\n{:?}", err);
                    }
                }
            })
        })
    }
}
