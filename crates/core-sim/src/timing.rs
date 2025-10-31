// Timing model for IBM 1130 devices

#[derive(Debug, Clone, Copy)]
pub struct TimingModel {
    enabled: bool,
    speed_multiplier: f64,
}

impl TimingModel {
    pub fn new(enabled: bool, speed_multiplier: f64) -> Self {
        Self {
            enabled,
            speed_multiplier,
        }
    }

    pub fn none() -> Self {
        Self {
            enabled: false,
            speed_multiplier: 1.0,
        }
    }

    pub fn realistic() -> Self {
        Self {
            enabled: true,
            speed_multiplier: 1.0,
        }
    }

    pub fn fast(multiplier: f64) -> Self {
        Self {
            enabled: true,
            speed_multiplier: multiplier,
        }
    }

    pub fn delay_us(&self, us: u64) -> u64 {
        if self.enabled {
            (us as f64 / self.speed_multiplier) as u64
        } else {
            0
        }
    }
}

impl Default for TimingModel {
    fn default() -> Self {
        Self::realistic()
    }
}
