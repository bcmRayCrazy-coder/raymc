use std::time::Duration;

use iced::Task;

use crate::{
    audio::track::{AudioTrack, AudioTrackType},
    ui::{
        app::App,
        message::{AudioMessage, Message},
    },
};

#[derive(Debug, Clone)]
pub enum AppAudioError {
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

    pub fn replay_track(&mut self, track_type: AudioTrackType) -> Result<(), AppAudioError> {
        let mut mixer = self
            .audio_manager
            .mixer
            .lock()
            .map_err(|_| AppAudioError::MutexLocked)?;
        for track in mixer.filter_track_mut(track_type).iter_mut() {
            track.replay();
        }
        Ok(())
    }

    pub fn update_audio(&mut self, message: AudioMessage) -> Task<Message> {
        match message {
            AudioMessage::PlayUi(name) => {
                let _ = self
                    .replay_track(AudioTrackType::UI(name))
                    .inspect_err(|err| eprintln!("Unable to play UI Audio! {:?}", err));
                Task::none()
            }
            AudioMessage::UpdatePlayerSong => {
                let mixer = self.audio_manager.mixer.lock();
                match mixer {
                    Ok(mut mixer) => {
                        mixer.remove_tracks(AudioTrackType::PLAYER);
                        if let Some(current) = self.player_manager.current {
                            let file = &self.player_manager.playlist[current];
                            mixer.add_track(
                                AudioTrack::from_disk(
                                    &file.to_str().expect("Audio load failed"),
                                    AudioTrackType::PLAYER,
                                )
                                .expect("Audio load failed"),
                            );
                        }
                        Task::none()
                    }
                    Err(_) => Task::perform(tokio::time::sleep(Duration::from_millis(1)), |_| {
                        Message::Audio(AudioMessage::UpdatePlayerSong)
                    }),
                }
            }
            AudioMessage::PlayerPlay => {
                let mixer = self.audio_manager.mixer.lock();
                match mixer {
                    Ok(mut mixer) => {
                        for track in mixer.filter_track_mut(AudioTrackType::PLAYER).iter_mut() {
                            if track.is_end() {
                                track.replay();
                            } else {
                                track.set_playing(true)
                            }
                        }
                        Task::none()
                    }
                    Err(_) => Task::perform(tokio::time::sleep(Duration::from_millis(1)), |_| {
                        Message::Audio(AudioMessage::UpdatePlayerSong)
                    }),
                }
            }
            AudioMessage::PlayerPause => {
                let mixer = self.audio_manager.mixer.lock();
                match mixer {
                    Ok(mut mixer) => {
                        for track in mixer.filter_track_mut(AudioTrackType::PLAYER).iter_mut() {
                            track.set_playing(false);
                        }
                        Task::none()
                    }
                    Err(_) => Task::perform(tokio::time::sleep(Duration::from_millis(1)), |_| {
                        Message::Audio(AudioMessage::UpdatePlayerSong)
                    }),
                }
            }
        }
    }
}
