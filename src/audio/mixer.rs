use std::collections::HashMap;

use iced::futures::channel::mpsc;

use crate::{
    audio::track::{AudioTrack, AudioTrackType},
    ui::message::{AudioMessage, Message},
};

#[derive(Debug)]
pub struct AudioMixer {
    pub tracks: HashMap<AudioTrackType, Box<AudioTrack>>,
    sample_rate: u32,
    sender: Option<mpsc::Sender<Message>>,
}

impl AudioMixer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            tracks: HashMap::new(),
            sample_rate,
            sender: None,
        }
    }

    pub fn tick_sample(&mut self) -> [f32; 2] {
        let mut sample0 = 0.0;
        let mut sample1 = 0.0;

        for (_, track) in self.tracks.iter_mut() {
            let (sample, ending) = track.tick_sample(self.sample_rate);
            sample0 += sample[0];
            sample1 += sample[1];
            if ending
                && let Some(sender) = self.sender.as_mut()
                && let Err(_) =
                    sender.try_send(Message::Audio(AudioMessage::TrackEnd(track.track_type())))
            {
                eprintln!("Ubable to send audio track end message");
            }
        }

        [sample0, sample1]
    }

    pub fn add_track_vec(&mut self, tracks: Vec<AudioTrack>) {
        for track in tracks {
            self.add_track(track);
        }
    }

    pub fn add_track(&mut self, track: AudioTrack) {
        self.tracks
            .insert(track.track_type().clone(), Box::new(track));
    }

    pub fn sender(&mut self, sender: mpsc::Sender<Message>) {
        self.sender = Some(sender);
    }
}
