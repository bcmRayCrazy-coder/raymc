#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudioTrackType {
    UI(&'static str),
    PLAYER,

    #[allow(unused)]
    TEST,
}

#[derive(Debug, Clone)]
pub struct AudioTrack {
    track_type: AudioTrackType,
    sample: Vec<[f32; 2]>,
    sample_rate: u32,
    is_playing: bool,
    pub stop_pos: usize,
    pub current_pos: f32,
    pub volume: f32,
}

impl AudioTrack {
    pub fn new(track_type: AudioTrackType, sample: Vec<[f32; 2]>, sample_rate: u32) -> Self {
        Self {
            stop_pos: sample.len(),
            current_pos: 0.0,
            volume: 1.0,

            track_type,
            sample,
            sample_rate,
            is_playing: false,
        }
    }

    pub fn tick_sample(&mut self, target_sample_rate: u32) -> [f32; 2] {
        if !self.is_playing() {
            return [0.0; 2];
        }
        if self.is_end() {
            self.set_playing(false);
            return [0.0; 2];
        }
        let pos = self.current_pos.floor() as usize;
        let sample = self.sample[pos].map(|s| s * self.volume);
        self.current_pos += self.sample_rate as f32 / target_sample_rate as f32;
        return sample;
    }

    pub fn track_type(&self) -> AudioTrackType {
        self.track_type.clone()
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn is_end(&self) -> bool {
        self.current_pos >= self.stop_pos as f32
    }

    pub fn set_playing(&mut self, play: bool) {
        self.is_playing = play;
    }

    pub fn replay(&mut self) {
        self.current_pos = 0.0;
        self.is_playing = true;
    }
}
