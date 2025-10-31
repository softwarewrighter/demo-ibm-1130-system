// IBM 1403 Line Printer (Model 6: 340 lpm, Model 7: 600 lpm)

use super::*;
use crate::timing::TimingModel;

pub struct Ibm1403 {
    timing: TimingModel,
    model_7: bool, // true for Model 7 (600 lpm), false for Model 6 (340 lpm)
    output: Vec<Vec<u8>>,
    status: DeviceStatusWord,
    line_width: usize, // 120 or 132 columns
}

impl Ibm1403 {
    pub fn new(model_7: bool, timing: TimingModel) -> Self {
        Self {
            timing,
            model_7,
            output: Vec::new(),
            status: DeviceStatusWord::ready(),
            line_width: 120,
        }
    }

    fn calculate_print_time_us(&self) -> u64 {
        let lpm = if self.model_7 { 600.0 } else { 340.0 };
        let time_per_line_ms = 60_000.0 / lpm;
        self.timing.delay_us((time_per_line_ms * 1000.0) as u64)
    }
}

impl Device for Ibm1403 {
    fn reset(&mut self) {
        self.output.clear();
        self.status = DeviceStatusWord::ready();
    }

    fn poll(&mut self, _now_us: u64) {
        // TODO: advance any pending operations
    }

    fn dsw(&self) -> DeviceStatusWord {
        self.status
    }
}

impl LinePrinter for Ibm1403 {
    fn print_line(&mut self, line: &[u8]) -> IoResult<()> {
        let mut padded_line = line.to_vec();
        padded_line.resize(self.line_width, b' ');

        self.output.push(padded_line);

        let _delay = self.calculate_print_time_us();

        Ok(())
    }

    fn get_output(&self) -> &[Vec<u8>] {
        &self.output
    }

    fn clear_output(&mut self) {
        self.output.clear();
    }
}
