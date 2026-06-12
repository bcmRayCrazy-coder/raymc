use std::time::Duration;

use iced::{
    Subscription, Task,
    futures::{SinkExt, StreamExt, channel::mpsc},
    stream,
};

use crate::{
    audio::track::{AudioTrack, AudioTrackType},
    ui::{
        app::App,
        message::{AudioMessage, Message, PlayerMessage, StateMessage},
    },
};

#[derive(Debug, Clone)]
pub enum AppAudioError {
    TrackNotFound,
    MutexLocked,
}

impl App {
    pub fn boot_audio(&mut self) {
        self.audio_manager
            .build_stream()
            .expect("Unable to build audio stream");
        let audio_stream = self.audio_manager.stream.clone();
        let audio_mixer = self.audio_manager.mixer.clone();
        std::thread::spawn(move || {
            let stream = audio_stream.lock().unwrap();
            let mut mixer = audio_mixer.lock().unwrap();

            mixer.add_track_vec(vec![
                AudioTrack::from_embed("audio/chord.wav", AudioTrackType::UI("chord")).unwrap(),
                AudioTrack::from_embed("audio/info.wav", AudioTrackType::UI("info")).unwrap(),
                AudioTrack::new(AudioTrackType::PLAYER, vec![[0.0, 0.0]], 1),
            ]);

            drop(mixer);

            match stream.play_stream() {
                Ok(()) => println!("Audio start"),
                Err(err) => eprintln!("Unable to start audio! {:?}", err),
            }
        });
    }

    pub fn replay_track(&mut self, track_type: &AudioTrackType) -> Result<(), AppAudioError> {
        let mut mixer = self
            .audio_manager
            .mixer
            .lock()
            .map_err(|_| AppAudioError::MutexLocked)?;

        match mixer.tracks.get_mut(track_type) {
            Some(track) => {
                track.replay();
                Ok(())
            }
            None => Err(AppAudioError::TrackNotFound),
        }
    }

    pub fn update_audio(&mut self, message: AudioMessage) -> Task<Message> {
        match message {
            AudioMessage::AudioMpscReady(sender) => {
                let mixer = self.audio_manager.mixer.clone();
                Task::future(async move {
                    loop {
                        match mixer.lock() {
                            Ok(mut mixer) => {
                                mixer.sender(sender);
                                break;
                            }
                            Err(_) => {}
                        };

                        tokio::time::sleep(Duration::from_millis(1)).await;
                        println!("Unlock failed retry after 1ms");
                    }
                    Message::None
                })
            }
            AudioMessage::TrackEnd(track_type) => match track_type {
                AudioTrackType::PLAYER => Task::done(Message::Player(PlayerMessage::PlayEnd)),
                _ => Task::none(),
            },

            AudioMessage::PlayUi(name) => {
                let _ = self
                    .replay_track(&AudioTrackType::UI(name))
                    .inspect_err(|err| eprintln!("Unable to play UI Audio! {:?}", err));
                Task::none()
            }
            AudioMessage::UpdatePlayerSong => {
                let mixer = self.audio_manager.mixer.clone();

                match self.player_manager.current() {
                    None => Task::perform::<bool>(
                        async move {
                            let unlocked = match mixer.lock() {
                                Ok(mut mixer) => {
                                    mixer.tracks.remove(&AudioTrackType::PLAYER);
                                    true
                                }
                                Err(_) => false,
                            };

                            if !unlocked {
                                tokio::time::sleep(Duration::from_millis(1)).await;
                            }

                            unlocked
                        },
                        |unlocked| match unlocked {
                            true => Message::None,
                            false => Message::Audio(AudioMessage::UpdatePlayerSong),
                        },
                    ),
                    Some(current) => {
                        let file = self.player_manager.playlist[current].clone();

                        Task::perform::<bool>(
                            async move {
                                let unlocked = match mixer.lock() {
                                    Ok(mut mixer) => {
                                        mixer.add_track(
                                            AudioTrack::from_disk(
                                                file.to_str().expect("Audio load failed"),
                                                AudioTrackType::PLAYER,
                                            )
                                            .expect("Audio load failed"),
                                        );
                                        true
                                    }
                                    Err(_) => false,
                                };

                                if !unlocked {
                                    tokio::time::sleep(Duration::from_millis(1)).await;
                                }

                                unlocked
                            },
                            |unlocked| match unlocked {
                                true => Message::Audio(AudioMessage::PlayerPlay),
                                false => Message::Audio(AudioMessage::UpdatePlayerSong),
                            },
                        )
                    }
                }
            }
            AudioMessage::PlayerPlay => {
                let mixer = self.audio_manager.mixer.lock();
                match mixer {
                    Ok(mut mixer) => {
                        if let Some(track) = mixer.tracks.get_mut(&AudioTrackType::PLAYER) {
                            if track.is_end() {
                                track.replay();
                            }
                            track.set_playing(true);
                            return Task::done(Message::State(StateMessage::OnPlayStateChanged(
                                true,
                            )));
                        } else {
                            if self.player_manager.current.is_some() {
                                return Task::batch([
                                    Task::done(Message::Audio(AudioMessage::UpdatePlayerSong)),
                                    Task::done(Message::State(StateMessage::OnPlayStateChanged(
                                        true,
                                    ))),
                                ]);
                            }
                        }
                        Task::none()
                    }
                    Err(_) => Task::perform(tokio::time::sleep(Duration::from_millis(1)), |_| {
                        Message::Audio(AudioMessage::PlayerPlay)
                    }),
                }
            }
            AudioMessage::PlayerPause => {
                let mixer = self.audio_manager.mixer.lock();
                match mixer {
                    Ok(mut mixer) => {
                        if let Some(track) = mixer.tracks.get_mut(&AudioTrackType::PLAYER) {
                            track.set_playing(false);
                        }
                        Task::done(Message::State(StateMessage::OnPlayStateChanged(false)))
                    }
                    Err(_) => Task::perform(tokio::time::sleep(Duration::from_millis(1)), |_| {
                        Message::Audio(AudioMessage::PlayerPause)
                    }),
                }
            }
        }
    }

    pub fn subscription_audio(&self) -> Subscription<Message> {
        Subscription::run(|| {
            stream::channel(127, async |mut output| {
                // output.send(Message::None).await;
                let (sender, mut reveiver) = mpsc::channel(127);
                output
                    .send(Message::Audio(AudioMessage::AudioMpscReady(sender)))
                    .await
                    .expect("Unable to open mpsc channel");

                loop {
                    if let Some(input) = reveiver.next().await
                        && let Err(err) = output.send(input).await
                    {
                        eprintln!("Failed to send message from audio: {:?}", err);
                    }
                }
            })
        })
    }
}
