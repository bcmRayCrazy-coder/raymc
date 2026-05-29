#[derive(Debug, Clone, PartialEq)]
pub enum AudioPlayType {
    TEST,
}

#[derive(Debug, Clone)]
pub struct AudioPlay {
    play_type: AudioPlayType,
    sample: Vec<[f32; 2]>,
    sample_rate: u32,
    is_playing: bool,
    pub stop_pos: usize,
    pub current_pos: usize,
    pub volume: f32,
}

impl AudioPlay {
    pub fn new(play_type: AudioPlayType, sample: Vec<[f32; 2]>, sample_rate: u32) -> Self {
        Self {
            stop_pos: sample.len(),
            current_pos: 0,
            volume: 1.0,

            play_type,
            sample,
            sample_rate,
            is_playing: false,
        }
    }

    pub fn tick_sample(&mut self) -> [f32; 2] {
        if !self.is_playing() {
            return [0.0; 2];
        }
        if self.is_end() {
            self.set_playing(false);
            return [0.0; 2];
        }
        let sample = self.sample[self.current_pos].map(|s| s * self.volume);
        self.current_pos += 1;
        return sample;
    }

    pub fn play_type(&self) -> &AudioPlayType {
        &self.play_type
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn is_end(&self) -> bool {
        self.current_pos >= self.stop_pos
    }

    pub fn set_playing(&mut self, play: bool) {
        self.is_playing = play;
    }
}
