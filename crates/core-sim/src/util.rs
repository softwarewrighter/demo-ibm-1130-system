// Utility functions for IBM 1130 simulation

/// Parity calculation for IBM 1130 words
/// Returns 4 check bits using modulo-4 parity over 16 data bits
pub fn calculate_parity(data: u16) -> u8 {
    // TODO: implement modulo-4 parity calculation
    // Placeholder: simple even parity for now
    data.count_ones() as u8 % 2
}

/// Pack a 16-bit data word into a 20-bit word with parity
pub fn pack_word(data: u16) -> u32 {
    let parity = calculate_parity(data);
    ((parity as u32) << 16) | (data as u32)
}

/// Unpack a 20-bit word and verify parity
pub fn unpack_word(packed: u32) -> Result<u16, ParityError> {
    let data = (packed & 0xFFFF) as u16;
    let stored_parity = ((packed >> 16) & 0xF) as u8;
    let calculated_parity = calculate_parity(data);

    if stored_parity == calculated_parity {
        Ok(data)
    } else {
        Err(ParityError {
            data,
            expected: calculated_parity,
            found: stored_parity,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParityError {
    pub data: u16,
    pub expected: u8,
    pub found: u8,
}

impl std::fmt::Display for ParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parity error: data={:04x}, expected={}, found={}",
            self.data, self.expected, self.found
        )
    }
}

impl std::error::Error for ParityError {}
