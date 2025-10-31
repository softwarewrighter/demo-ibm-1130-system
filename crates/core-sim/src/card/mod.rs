// IBM 1442 Card Reader/Punch

pub mod ibm1442;

use crate::disk::{Device, DeviceStatusWord, IoError, IoResult};

/// 80-column card with 12-row punch encoding
#[derive(Debug, Clone)]
pub struct Card80 {
    /// 12 rows x 80 columns, stored as 80 u16 values (12 bits used per column)
    pub columns: [u16; 80],
}

impl Card80 {
    pub fn new() -> Self {
        Self { columns: [0; 80] }
    }

    pub fn from_ascii(text: &str) -> Self {
        let mut card = Self::new();
        for (i, ch) in text.chars().take(80).enumerate() {
            card.columns[i] = Self::ascii_to_hollerith(ch);
        }
        card
    }

    pub fn to_ascii(&self) -> String {
        self.columns
            .iter()
            .map(|&col| Self::hollerith_to_ascii(col))
            .collect()
    }

    fn ascii_to_hollerith(ch: char) -> u16 {
        // TODO: implement proper EBCDIC/Hollerith encoding
        // Placeholder: just encode printable ASCII
        match ch {
            ' ' => 0,
            _ => ch as u16 & 0x0FFF,
        }
    }

    fn hollerith_to_ascii(code: u16) -> char {
        // TODO: implement proper Hollerith/ASCII decoding
        if code == 0 {
            ' '
        } else {
            ((code & 0x7F) as u8) as char
        }
    }
}

impl Default for Card80 {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CardStatus {
    pub hopper_count: usize,
    pub stacker_a_count: usize,
    pub stacker_b_count: usize,
    pub read_ready: bool,
    pub punch_ready: bool,
}

pub trait CardDevice: Device {
    fn hopper_load(&mut self, deck: Vec<Card80>);
    fn read_card(&mut self) -> IoResult<Card80>;
    fn punch_card(&mut self, card: &Card80, to_stacker_b: bool) -> IoResult<()>;
    fn status(&self) -> CardStatus;
}
