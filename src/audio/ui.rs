use std::sync::PoisonError;

use iced::Task;

use crate::{
    audio::track::{AudioTrack, AudioTrackType},
    ui::{app::App, message::{AudioMessage, Message}},
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

            // let mut test_sample =
            //     AudioTrack::from_embed("test/test.wav", AudioTrackType::TEST).unwrap();
            // // AudioTrack::from_embed("audio/chord.wav", AudioTrackType::TEST).unwrap();
            // test_sample.set_playing(true);
            // mixer.add_track(test_sample);
            mixer.add_track(
                AudioTrack::from_embed("audio/chord.wav", AudioTrackType::UI("chord")).unwrap(),
            );

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
            AudioMessage::PlayUi(name)=>{
                let _ = self.replay_track(AudioTrackType::UI(name)).inspect_err(|err| eprintln!("Unable to play UI Audio! {:?}", err));
                Task::none()
            }
            _=>Task::none()
        }
    }
}
