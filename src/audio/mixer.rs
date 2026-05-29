use iced::widget::sensor::Key;

use crate::audio::track::{AudioTrack, AudioTrackType};

#[derive(Debug)]
pub struct AudioMixer {
    tracks: Vec<AudioTrack>,
}

impl AudioMixer {
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    pub fn tick_sample(&mut self) -> [f32; 2] {
        let mut sample0 = 0.0;
        let mut sample1 = 0.0;

        for play in self.tracks.iter_mut() {
            let sample = play.tick_sample();
            sample0 += sample[0];
            sample1 += sample[1];
        }

        [sample0, sample1]
    }

    pub fn trakcs(&self) -> &Vec<AudioTrack> {
        &self.tracks
    }

    pub fn filter_track(&self, track_type: AudioTrackType) -> Vec<&AudioTrack> {
        self.tracks
            .iter()
            .filter(|t| t.track_type().eq(&track_type))
            .collect()
    }

    pub fn add_track(&mut self, play: AudioTrack) {
        self.tracks.push(play);
    }

    pub fn remove_track(&mut self, id: usize) -> bool {
        if id >= self.tracks.len() {
            return false;
        }
        self.tracks.remove(id);
        true
    }
}
