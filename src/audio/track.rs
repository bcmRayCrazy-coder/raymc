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
    frac: f64,
    pub stop_pos: usize,
    pub current_pos: usize,
    pub volume: f32,
}

impl AudioTrack {
    pub fn new(track_type: AudioTrackType, sample: Vec<[f32; 2]>, sample_rate: u32) -> Self {
        Self {
            stop_pos: sample.len(),
            current_pos: 0,
            volume: 1.0,
            frac: 0.0,

            track_type,
            sample,
            sample_rate,
            is_playing: false,
        }
    }

    pub fn tick_sample(&mut self, target_sample_rate: u32) -> ([f32; 2], bool) {
        if !self.is_playing() {
            return ([0.0; 2], false);
        }
        if self.is_end() {
            self.set_playing(false);
            return ([0.0; 2], true);
        }

        let next_pos = self.current_pos + 1;
        let next = if next_pos >= self.sample.len() {
            [0.0, 0.0]
        } else {
            self.sample[next_pos]
        };

        let mut sample = self.sample[self.current_pos];
        sample[0] = sample[0] * self.frac as f32 + next[0] - next[0] * self.frac as f32;
        sample[1] = sample[1] * self.frac as f32 + next[1] - next[1] * self.frac as f32;
        let step = self.sample_rate as f64 / target_sample_rate as f64;
        self.frac += step;
        if self.frac >= 1.0 {
            let floor = self.frac.floor();
            self.current_pos += floor as usize;
            self.frac -= floor;
        }
        return (sample.map(|s| s * self.volume), false);
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
        self.current_pos >= self.stop_pos
    }

    pub fn set_playing(&mut self, play: bool) {
        self.is_playing = play;
    }

    pub fn replay(&mut self) {
        self.current_pos = 0;
        self.is_playing = true;
    }
}
