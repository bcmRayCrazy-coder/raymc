use iced::widget::sensor::Key;

use crate::audio::track::{AudioTrack, AudioTrackType};

#[derive(Debug)]
pub struct AudioMixer {
    tracks: Vec<AudioTrack>,
    sample_rate: u32,
}

impl AudioMixer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            tracks: Vec::new(),
            sample_rate,
        }
    }

    pub fn tick_sample(&mut self) -> [f32; 2] {
        let mut sample0 = 0.0;
        let mut sample1 = 0.0;

        for play in self.tracks.iter_mut() {
            let sample = play.tick_sample(self.sample_rate);
            sample0 += sample[0];
            sample1 += sample[1];
        }

        [sample0, sample1]
    }

    pub fn trakcs(&self) -> &Vec<AudioTrack> {
        &self.tracks
    }

    pub fn filter_track_mut(&mut self, track_type: AudioTrackType) -> Vec<&mut AudioTrack> {
        self.tracks
            .iter_mut()
            .filter(|t| t.track_type().eq(&track_type))
            .collect()
    }

    pub fn add_track_vec(&mut self, tracks: Vec<AudioTrack>) {
        for track in tracks {
            self.add_track(track);
        }
    }

    pub fn add_track(&mut self, track: AudioTrack) {
        self.tracks.push(track);
    }

    pub fn remove_track(&mut self, id: usize) -> bool {
        if id >= self.tracks.len() {
            return false;
        }
        self.tracks.remove(id);
        true
    }
}
