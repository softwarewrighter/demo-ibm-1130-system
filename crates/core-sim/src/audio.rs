// Audio synthesis model for seek sounds

#[derive(Debug, Clone, Copy)]
pub struct SeekProfile {
    pub delta_cyl: i16,
    pub accel_time_ms: f64,
    pub settle_time_ms: f64,
}

impl SeekProfile {
    pub fn new(delta_cyl: i16) -> Self {
        let abs_delta = delta_cyl.abs() as f64;
        Self {
            delta_cyl,
            accel_time_ms: abs_delta * 7.5,
            settle_time_ms: 22.5,
        }
    }

    pub fn total_time_ms(&self) -> f64 {
        self.accel_time_ms + self.settle_time_ms
    }
}

pub struct AudioModel {
    // TODO: WebAudio integration parameters
}

impl AudioModel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_seek_sound(&self, _profile: SeekProfile) {
        // TODO: generate pitch sweep based on seek profile
    }
}

impl Default for AudioModel {
    fn default() -> Self {
        Self::new()
    }
}
