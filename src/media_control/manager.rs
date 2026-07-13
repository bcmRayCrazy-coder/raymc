use std::{fmt::Debug, path::PathBuf, sync::Arc};

use iced::futures::{SinkExt, channel::mpsc};
use souvlaki::{MediaControls, MediaMetadata, MediaPlayback, PlatformConfig};
use tokio::sync::Mutex;

use crate::{
    player::song::PlaySong,
    ui::message::{AudioMessage, Message, PlayerMessage},
};

fn generate_cover_url(path: &PathBuf) -> Option<&str> {
    let str_path = path.to_str()?;

    #[cfg(target_os = "windows")]
    {
        return Some(("file://".to_owned() + str_path).leak());
    }

    #[cfg(target_os = "linux")]
    {
        return Some(("file://".to_owned() + str_path).leak());
    }

    // Unimplement: MacOS

    #[allow(unused)]
    None
}

#[derive(Debug)]
#[allow(unused)]
pub enum MediaControlError {
    ControlsError,
    AttachError(souvlaki::Error),
    DetachError(souvlaki::Error),
    SetMetadataError(souvlaki::Error),
    SetPlayingError(souvlaki::Error),
}

pub struct MediaControlManager {
    sender: Arc<Mutex<mpsc::Sender<Message>>>,
    controls: Box<MediaControls>,
}

impl MediaControlManager {
    pub fn new(sender: mpsc::Sender<Message>) -> Result<Self, MediaControlError> {
        let config = PlatformConfig {
            display_name: "Ray Music Center",
            dbus_name: "raymc",
            hwnd: None,
        };

        Ok(Self {
            sender: Arc::new(Mutex::new(sender)),
            controls: Box::new(
                MediaControls::new(config).map_err(|_| MediaControlError::ControlsError)?,
            ),
        })
    }

    pub fn set_song(&mut self, song: PlaySong) -> Result<(), MediaControlError> {
        let cover_url = match song.cover.as_ref() {
            None => None,
            Some(path) => generate_cover_url(path),
        };

        self.controls
            .set_metadata(MediaMetadata {
                title: Some(song.name()),
                cover_url: cover_url,
                ..Default::default()
            })
            .map_err(|err| MediaControlError::SetMetadataError(err))
    }

    pub fn set_playing(&mut self, is_playing: bool) -> Result<(), MediaControlError> {
        self.controls
            .set_playback(match is_playing {
                true => MediaPlayback::Playing { progress: None },
                false => MediaPlayback::Paused { progress: None },
            })
            .map_err(|err| MediaControlError::SetPlayingError(err))
    }

    pub fn attach(&mut self) -> Result<(), MediaControlError> {
        let sender = self.sender.clone();
        self.controls
            .attach(move |event| {
                println!("Control event {:?}", event);

                let sender = sender.clone();
                std::thread::spawn(move || {
                    if let Ok(runtime) = tokio::runtime::Runtime::new() {
                        runtime.spawn(async move {
                            let mut sender = sender.lock().await;
                            match event {
                                souvlaki::MediaControlEvent::Play => {
                                    let _ =
                                        sender.send(Message::Audio(AudioMessage::PlayerPlay)).await;
                                }
                                souvlaki::MediaControlEvent::Pause => {
                                    let _ = sender
                                        .send(Message::Audio(AudioMessage::PlayerPause))
                                        .await;
                                }
                                souvlaki::MediaControlEvent::Toggle => {
                                    let _ = sender
                                        .send(Message::Audio(AudioMessage::PlayerToggle))
                                        .await;
                                }
                                souvlaki::MediaControlEvent::Next => {
                                    let _ =
                                        sender.send(Message::Player(PlayerMessage::ListNext)).await;
                                }
                                souvlaki::MediaControlEvent::Previous => {
                                    let _ =
                                        sender.send(Message::Player(PlayerMessage::ListPrev)).await;
                                }
                                _ => {}
                            };
                        });
                    }
                });
            })
            .map_err(|err| MediaControlError::AttachError(err))
    }

    pub fn detach(&mut self) -> Result<(), MediaControlError> {
        self.controls
            .detach()
            .map_err(|err| MediaControlError::DetachError(err))
    }
}
