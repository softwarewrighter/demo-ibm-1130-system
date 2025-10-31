// IBM 1442 Card Read Punch

use super::*;
use crate::timing::TimingModel;

pub struct Ibm1442 {
    timing: TimingModel,
    hopper: Vec<Card80>,
    stacker_a: Vec<Card80>,
    stacker_b: Vec<Card80>,
    status: DeviceStatusWord,
    read_speed_cpm: u32,  // cards per minute (up to 400)
    punch_speed_cpm: u32, // cards per minute (model dependent, up to ~360)
}

impl Ibm1442 {
    pub fn new(timing: TimingModel) -> Self {
        Self {
            timing,
            hopper: Vec::new(),
            stacker_a: Vec::new(),
            stacker_b: Vec::new(),
            status: DeviceStatusWord {
                busy: false,
                error: false,
                attention: false,
            },
            read_speed_cpm: 400,
            punch_speed_cpm: 360,
        }
    }

    fn calculate_read_time_us(&self) -> u64 {
        let time_per_card_ms = 60_000.0 / self.read_speed_cpm as f64;
        self.timing.delay_us((time_per_card_ms * 1000.0) as u64)
    }

    fn calculate_punch_time_us(&self) -> u64 {
        let time_per_card_ms = 60_000.0 / self.punch_speed_cpm as f64;
        self.timing.delay_us((time_per_card_ms * 1000.0) as u64)
    }
}

impl Device for Ibm1442 {
    fn reset(&mut self) {
        self.status = DeviceStatusWord {
            busy: false,
            error: false,
            attention: false,
        };
    }

    fn poll(&mut self, _now_us: u64) {
        // TODO: advance any pending operations
    }

    fn dsw(&self) -> DeviceStatusWord {
        self.status
    }
}

impl CardDevice for Ibm1442 {
    fn hopper_load(&mut self, mut deck: Vec<Card80>) {
        self.hopper.append(&mut deck);
    }

    fn read_card(&mut self) -> IoResult<Card80> {
        if self.hopper.is_empty() {
            return Err(IoError::ReadError);
        }

        let card = self.hopper.remove(0);
        self.stacker_a.push(card.clone());

        let _delay = self.calculate_read_time_us();

        Ok(card)
    }

    fn punch_card(&mut self, card: &Card80, to_stacker_b: bool) -> IoResult<()> {
        let _delay = self.calculate_punch_time_us();

        if to_stacker_b {
            self.stacker_b.push(card.clone());
        } else {
            self.stacker_a.push(card.clone());
        }

        Ok(())
    }

    fn status(&self) -> CardStatus {
        CardStatus {
            hopper_count: self.hopper.len(),
            stacker_a_count: self.stacker_a.len(),
            stacker_b_count: self.stacker_b.len(),
            read_ready: !self.hopper.is_empty(),
            punch_ready: true,
        }
    }
}
