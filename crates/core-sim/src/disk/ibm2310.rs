//! IBM 2310 Disk Storage Drive (uses 2315 cartridge).
//!
//! The IBM 2310 is a single-drive disk storage unit using the IBM 2315
//! removable disk cartridge. This module provides a complete simulation
//! with accurate timing, geometry, and addressing.
//!
//! # Specifications
//!
//! - **Capacity**: 512,000 16-bit words (~1 MB)
//! - **Cylinders**: 200 logical (203 physical with 3 alternates)
//! - **Heads**: 2 (top and bottom surfaces)
//! - **Sectors per track**: 4
//! - **Words per sector**: 321 (1 address word + 320 data words)
//! - **RPM**: 1500 (40ms per revolution)
//! - **Word transfer rate**: 27.8 microseconds per word
//! - **Seek time**: 7.5ms per 2-cylinder increment + 22.5ms settle
//!
//! # Examples
//!
//! ```
//! use core_sim::api::make_2315;
//! use core_sim::TimingModel;
//! use core_sim::disk::DiskDevice;
//!
//! let mut disk = make_2315(TimingModel::none());
//!
//! // Seek to cylinder 100
//! let outcome = disk.seek(100);
//! assert_eq!(outcome.quantized_cyl, 100);
//!
//! // Read a sector
//! let mut buf = [0u16; 321];
//! disk.read_sector(100, 0, 0, &mut buf).unwrap();
//! ```

use super::*;

pub struct Ibm2310 {
    geometry: Geometry,
    timing: TimingModel,
    current_cyl: u16,
    current_head: u8,
    status: DeviceStatusWord,
    data: Vec<u16>,
}

impl Ibm2310 {
    pub fn new(timing: TimingModel) -> Self {
        let geometry = Geometry::IBM2315;
        let total_words = geometry.total_words();

        Self {
            geometry,
            timing,
            current_cyl: 0,
            current_head: 0,
            status: DeviceStatusWord::ready(),
            data: vec![0u16; total_words],
        }
    }

    pub fn load_image(&mut self, data: Vec<u16>) -> Result<(), &'static str> {
        if data.len() != self.geometry.total_words() {
            return Err("Image size mismatch");
        }
        self.data = data;
        Ok(())
    }

    pub fn get_image(&self) -> &[u16] {
        &self.data
    }

    fn quantize_cylinder(&self, cyl: u16) -> u16 {
        // 2315 seeks in increments of 2 cylinders
        (cyl / 2) * 2
    }

    fn calculate_seek_time(&self, from: u16, to: u16) -> u64 {
        let delta = ((to as i32 - from as i32).abs() / 2) as f64; // N_even
        let time_ms = delta * 7.5 + 22.5; // 7.5ms per even increment + 22.5ms settle
        self.timing.delay_us((time_ms * 1000.0) as u64)
    }

    fn sector_to_offset(&self, cyl: u16, head: u8, sector: u8) -> Option<usize> {
        if cyl >= self.geometry.cylinders
            || head >= self.geometry.heads
            || sector >= self.geometry.sectors_per_track
        {
            return None;
        }

        let sectors_per_cyl =
            self.geometry.heads as usize * self.geometry.sectors_per_track as usize;
        let sector_index = cyl as usize * sectors_per_cyl
            + head as usize * self.geometry.sectors_per_track as usize
            + sector as usize;

        Some(sector_index * self.geometry.words_per_sector as usize)
    }
}

impl Device for Ibm2310 {
    fn reset(&mut self) {
        self.current_cyl = 0;
        self.current_head = 0;
        self.status = DeviceStatusWord::ready();
    }

    fn poll(&mut self, _now_us: u64) {
        // TODO: advance any pending operations
    }

    fn dsw(&self) -> DeviceStatusWord {
        self.status
    }
}

impl DiskDevice for Ibm2310 {
    fn geometry(&self) -> Geometry {
        self.geometry
    }

    fn seek(&mut self, cyl: u16) -> SeekOutcome {
        let quantized = self.quantize_cylinder(cyl);
        let time_us = self.calculate_seek_time(self.current_cyl, quantized);
        self.current_cyl = quantized;

        SeekOutcome {
            target_cyl: cyl,
            quantized_cyl: quantized,
            time_us,
        }
    }

    fn select_head(&mut self, head: u8) {
        if head < self.geometry.heads {
            self.current_head = head;
        }
    }

    fn read_sector(
        &mut self,
        cyl: u16,
        head: u8,
        sector: u8,
        buf: &mut [u16; 321],
    ) -> IoResult<()> {
        let offset = self
            .sector_to_offset(cyl, head, sector)
            .ok_or(IoError::InvalidAddress)?;

        let sector_data = &self.data[offset..offset + 321];
        buf.copy_from_slice(sector_data);

        // Simulate rotational latency
        let _delay = self.timing.delay_us(20_000); // avg 20ms

        Ok(())
    }

    fn write_sector(&mut self, cyl: u16, head: u8, sector: u8, buf: &[u16; 321]) -> IoResult<()> {
        let offset = self
            .sector_to_offset(cyl, head, sector)
            .ok_or(IoError::InvalidAddress)?;

        self.data[offset..offset + 321].copy_from_slice(buf);

        // Simulate write time
        let _delay = self.timing.delay_us(8_900); // ~8.9ms for 321 words

        Ok(())
    }

    fn read_block20(&mut self, addr: BlockAddr, buf: &mut [u16; 20]) -> IoResult<()> {
        let offset = self
            .sector_to_offset(addr.cyl, addr.head, addr.sector)
            .ok_or(IoError::InvalidAddress)?;

        let word_offset = addr.word_offset_in_sector();
        let block_data = &self.data[offset + word_offset..offset + word_offset + 20];
        buf.copy_from_slice(block_data);

        Ok(())
    }

    fn write_block20(&mut self, addr: BlockAddr, buf: &[u16; 20]) -> IoResult<()> {
        let offset = self
            .sector_to_offset(addr.cyl, addr.head, addr.sector)
            .ok_or(IoError::InvalidAddress)?;

        let word_offset = addr.word_offset_in_sector();
        self.data[offset + word_offset..offset + word_offset + 20].copy_from_slice(buf);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor and Initialization Tests ==========

    #[test]
    fn test_ibm2310_new_initializes_correctly() {
        let disk = Ibm2310::new(TimingModel::none());
        assert_eq!(disk.geometry, Geometry::IBM2315);
        assert_eq!(disk.current_cyl, 0);
        assert_eq!(disk.current_head, 0);
        assert_eq!(disk.data.len(), 513_600); // 200*2*4*321
    }

    #[test]
    fn test_ibm2310_reset() {
        let mut disk = Ibm2310::new(TimingModel::none());
        disk.current_cyl = 100;
        disk.current_head = 1;

        disk.reset();

        assert_eq!(disk.current_cyl, 0);
        assert_eq!(disk.current_head, 0);
        let dsw = disk.dsw();
        assert!(dsw.ready);
        assert!(!dsw.busy);
    }

    #[test]
    fn test_ibm2310_load_image_success() {
        let mut disk = Ibm2310::new(TimingModel::none());
        let data = vec![0xABCDu16; 513_600];

        let result = disk.load_image(data.clone());
        assert!(result.is_ok());
        assert_eq!(disk.get_image()[0], 0xABCD);
        assert_eq!(disk.get_image().len(), 513_600);
    }

    #[test]
    fn test_ibm2310_load_image_wrong_size_fails() {
        let mut disk = Ibm2310::new(TimingModel::none());
        let data = vec![0u16; 1000]; // Wrong size

        let result = disk.load_image(data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Image size mismatch");
    }

    // ========== Cylinder Quantization Tests ==========

    #[test]
    fn test_quantize_cylinder_even() {
        let disk = Ibm2310::new(TimingModel::none());
        assert_eq!(disk.quantize_cylinder(0), 0);
        assert_eq!(disk.quantize_cylinder(2), 2);
        assert_eq!(disk.quantize_cylinder(100), 100);
        assert_eq!(disk.quantize_cylinder(198), 198);
    }

    #[test]
    fn test_quantize_cylinder_odd_rounds_down() {
        let disk = Ibm2310::new(TimingModel::none());
        assert_eq!(disk.quantize_cylinder(1), 0);
        assert_eq!(disk.quantize_cylinder(3), 2);
        assert_eq!(disk.quantize_cylinder(99), 98);
        assert_eq!(disk.quantize_cylinder(199), 198);
    }

    // ========== Seek Timing Tests ==========

    #[test]
    fn test_calculate_seek_time_same_cylinder() {
        let disk = Ibm2310::new(TimingModel::realistic());
        // Seek from 0 to 0: only settle time (22.5ms)
        let time_us = disk.calculate_seek_time(0, 0);
        assert_eq!(time_us, 22_500);
    }

    #[test]
    fn test_calculate_seek_time_one_increment() {
        let disk = Ibm2310::new(TimingModel::realistic());
        // Seek from 0 to 2: 1 increment * 7.5ms + 22.5ms settle = 30ms
        let time_us = disk.calculate_seek_time(0, 2);
        assert_eq!(time_us, 30_000);
    }

    #[test]
    fn test_calculate_seek_time_fifty_increments() {
        let disk = Ibm2310::new(TimingModel::realistic());
        // Seek from 0 to 100: 50 increments * 7.5ms + 22.5ms = 397.5ms
        let time_us = disk.calculate_seek_time(0, 100);
        assert_eq!(time_us, 397_500);
    }

    #[test]
    fn test_calculate_seek_time_full_span() {
        let disk = Ibm2310::new(TimingModel::realistic());
        // Seek from 0 to 198: 99 increments * 7.5ms + 22.5ms = 765ms
        let time_us = disk.calculate_seek_time(0, 198);
        assert_eq!(time_us, 765_000);
    }

    #[test]
    fn test_calculate_seek_time_with_none_timing() {
        let disk = Ibm2310::new(TimingModel::none());
        // TimingModel::none() should return 0
        let time_us = disk.calculate_seek_time(0, 100);
        assert_eq!(time_us, 0);
    }

    #[test]
    fn test_calculate_seek_time_with_fast_timing() {
        let disk = Ibm2310::new(TimingModel::fast(10.0));
        // 10x faster: 397.5ms / 10 = 39.75ms
        let time_us = disk.calculate_seek_time(0, 100);
        assert_eq!(time_us, 39_750);
    }

    // ========== Seek Operation Tests ==========

    #[test]
    fn test_seek_updates_current_cylinder() {
        let mut disk = Ibm2310::new(TimingModel::none());
        assert_eq!(disk.current_cyl, 0);

        disk.seek(100);
        assert_eq!(disk.current_cyl, 100);

        disk.seek(50);
        assert_eq!(disk.current_cyl, 50);
    }

    #[test]
    fn test_seek_quantizes_odd_cylinder() {
        let mut disk = Ibm2310::new(TimingModel::none());

        let outcome = disk.seek(99);
        assert_eq!(outcome.target_cyl, 99);
        assert_eq!(outcome.quantized_cyl, 98);
        assert_eq!(disk.current_cyl, 98);
    }

    #[test]
    fn test_seek_outcome_returns_correct_values() {
        let mut disk = Ibm2310::new(TimingModel::realistic());

        let outcome = disk.seek(50);
        assert_eq!(outcome.target_cyl, 50);
        assert_eq!(outcome.quantized_cyl, 50);
        assert_eq!(outcome.time_us, 210_000); // 25 * 7.5ms + 22.5ms
    }

    // ========== Head Selection Tests ==========

    #[test]
    fn test_select_head_valid() {
        let mut disk = Ibm2310::new(TimingModel::none());
        assert_eq!(disk.current_head, 0);

        disk.select_head(1);
        assert_eq!(disk.current_head, 1);

        disk.select_head(0);
        assert_eq!(disk.current_head, 0);
    }

    #[test]
    fn test_select_head_invalid_ignores() {
        let mut disk = Ibm2310::new(TimingModel::none());
        disk.select_head(0);
        assert_eq!(disk.current_head, 0);

        // Head 2 is invalid (only 0 and 1 exist)
        disk.select_head(2);
        assert_eq!(disk.current_head, 0); // Should remain unchanged
    }

    // ========== Sector Offset Calculation Tests ==========

    #[test]
    fn test_sector_to_offset_first_sector() {
        let disk = Ibm2310::new(TimingModel::none());
        let offset = disk.sector_to_offset(0, 0, 0);
        assert_eq!(offset, Some(0));
    }

    #[test]
    fn test_sector_to_offset_second_head() {
        let disk = Ibm2310::new(TimingModel::none());
        // Head 1, sector 0 should be at sector index 4 (4 sectors per head)
        let offset = disk.sector_to_offset(0, 1, 0);
        assert_eq!(offset, Some(4 * 321)); // 1284 words
    }

    #[test]
    fn test_sector_to_offset_second_cylinder() {
        let disk = Ibm2310::new(TimingModel::none());
        // Cyl 1, head 0, sector 0 = 8 sectors * 321 words
        let offset = disk.sector_to_offset(1, 0, 0);
        assert_eq!(offset, Some(8 * 321)); // 2568 words
    }

    #[test]
    fn test_sector_to_offset_invalid_cylinder() {
        let disk = Ibm2310::new(TimingModel::none());
        let offset = disk.sector_to_offset(200, 0, 0); // Cyl 200 is out of range
        assert_eq!(offset, None);
    }

    #[test]
    fn test_sector_to_offset_invalid_head() {
        let disk = Ibm2310::new(TimingModel::none());
        let offset = disk.sector_to_offset(0, 2, 0); // Only heads 0-1 valid
        assert_eq!(offset, None);
    }

    #[test]
    fn test_sector_to_offset_invalid_sector() {
        let disk = Ibm2310::new(TimingModel::none());
        let offset = disk.sector_to_offset(0, 0, 4); // Only sectors 0-3 valid
        assert_eq!(offset, None);
    }

    // ========== Read Sector Tests ==========

    #[test]
    fn test_read_sector_success() {
        let mut disk = Ibm2310::new(TimingModel::none());

        // Write test pattern to sector 0
        disk.data[0] = 0x1234;
        disk.data[1] = 0x5678;
        disk.data[320] = 0xABCD;

        let mut buf = [0u16; 321];
        let result = disk.read_sector(0, 0, 0, &mut buf);

        assert!(result.is_ok());
        assert_eq!(buf[0], 0x1234);
        assert_eq!(buf[1], 0x5678);
        assert_eq!(buf[320], 0xABCD);
    }

    #[test]
    fn test_read_sector_invalid_address() {
        let mut disk = Ibm2310::new(TimingModel::none());
        let mut buf = [0u16; 321];

        let result = disk.read_sector(200, 0, 0, &mut buf);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), IoError::InvalidAddress);
    }

    #[test]
    fn test_read_sector_from_different_cylinder() {
        let mut disk = Ibm2310::new(TimingModel::none());

        // Write pattern to cylinder 10, head 0, sector 0
        let offset = disk.sector_to_offset(10, 0, 0).unwrap();
        disk.data[offset] = 0xDEAD;
        disk.data[offset + 1] = 0xBEEF;

        let mut buf = [0u16; 321];
        let result = disk.read_sector(10, 0, 0, &mut buf);

        assert!(result.is_ok());
        assert_eq!(buf[0], 0xDEAD);
        assert_eq!(buf[1], 0xBEEF);
    }

    // ========== Write Sector Tests ==========

    #[test]
    fn test_write_sector_success() {
        let mut disk = Ibm2310::new(TimingModel::none());
        let mut buf = [0u16; 321];
        buf[0] = 0x1111;
        buf[100] = 0x2222;
        buf[320] = 0x3333;

        let result = disk.write_sector(0, 0, 0, &buf);
        assert!(result.is_ok());

        assert_eq!(disk.data[0], 0x1111);
        assert_eq!(disk.data[100], 0x2222);
        assert_eq!(disk.data[320], 0x3333);
    }

    #[test]
    fn test_write_sector_invalid_address() {
        let mut disk = Ibm2310::new(TimingModel::none());
        let buf = [0u16; 321];

        let result = disk.write_sector(0, 2, 0, &buf); // Invalid head
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), IoError::InvalidAddress);
    }

    #[test]
    fn test_write_then_read_sector_roundtrip() {
        let mut disk = Ibm2310::new(TimingModel::none());

        // Write pattern
        let write_buf = [0xAA55u16; 321];
        disk.write_sector(5, 1, 2, &write_buf).unwrap();

        // Read back
        let mut read_buf = [0u16; 321];
        disk.read_sector(5, 1, 2, &mut read_buf).unwrap();

        // Verify round-trip
        assert_eq!(write_buf, read_buf);
    }

    // ========== Read Block Tests ==========

    #[test]
    fn test_read_block20_first_block() {
        let mut disk = Ibm2310::new(TimingModel::none());

        // Block 0 starts at word 1 (word 0 is sector address)
        let offset = disk.sector_to_offset(0, 0, 0).unwrap();
        disk.data[offset + 1] = 0x1234; // First word of block 0
        disk.data[offset + 20] = 0x5678; // First word of block 1

        let addr = BlockAddr::new(0, 0, 0, 0);
        let mut buf = [0u16; 20];
        let result = disk.read_block20(addr, &mut buf);

        assert!(result.is_ok());
        assert_eq!(buf[0], 0x1234);
        assert_ne!(buf[0], 0x5678); // Should not include next block
    }

    #[test]
    fn test_read_block20_last_block() {
        let mut disk = Ibm2310::new(TimingModel::none());

        // Block 15 starts at word 301 (1 + 15*20)
        let offset = disk.sector_to_offset(0, 0, 0).unwrap();
        disk.data[offset + 301] = 0xABCD;

        let addr = BlockAddr::new(0, 0, 0, 15);
        let mut buf = [0u16; 20];
        let result = disk.read_block20(addr, &mut buf);

        assert!(result.is_ok());
        assert_eq!(buf[0], 0xABCD);
    }

    #[test]
    fn test_read_block20_invalid_address() {
        let mut disk = Ibm2310::new(TimingModel::none());
        let addr = BlockAddr::new(200, 0, 0, 0); // Invalid cylinder
        let mut buf = [0u16; 20];

        let result = disk.read_block20(addr, &mut buf);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), IoError::InvalidAddress);
    }

    // ========== Write Block Tests ==========

    #[test]
    fn test_write_block20_success() {
        let mut disk = Ibm2310::new(TimingModel::none());
        let buf = [0xDEADu16; 20];
        let addr = BlockAddr::new(0, 0, 0, 5);

        let result = disk.write_block20(addr, &buf);
        assert!(result.is_ok());

        // Verify written at correct offset (1 + 5*20 = 101)
        let offset = disk.sector_to_offset(0, 0, 0).unwrap();
        assert_eq!(disk.data[offset + 101], 0xDEAD);
        assert_eq!(disk.data[offset + 120], 0xDEAD); // Last word of block
    }

    #[test]
    fn test_write_then_read_block_roundtrip() {
        let mut disk = Ibm2310::new(TimingModel::none());

        let write_buf = [0xBEEFu16; 20];
        let addr = BlockAddr::new(10, 1, 3, 7);

        disk.write_block20(addr, &write_buf).unwrap();

        let mut read_buf = [0u16; 20];
        disk.read_block20(addr, &mut read_buf).unwrap();

        assert_eq!(write_buf, read_buf);
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_complete_workflow_seek_write_read() {
        let mut disk = Ibm2310::new(TimingModel::none());

        // 1. Seek to cylinder 50
        let seek_outcome = disk.seek(50);
        assert_eq!(seek_outcome.quantized_cyl, 50);

        // 2. Write a sector
        let write_buf = [0xCAFEu16; 321];
        disk.write_sector(50, 0, 2, &write_buf).unwrap();

        // 3. Seek away and back
        disk.seek(0);
        disk.seek(50);

        // 4. Read the sector back
        let mut read_buf = [0u16; 321];
        disk.read_sector(50, 0, 2, &mut read_buf).unwrap();

        // 5. Verify
        assert_eq!(write_buf, read_buf);
    }

    #[test]
    fn test_multiple_block_operations_same_sector() {
        let mut disk = Ibm2310::new(TimingModel::none());

        // Write different patterns to different blocks in same sector
        let addr0 = BlockAddr::new(0, 0, 0, 0);
        let addr1 = BlockAddr::new(0, 0, 0, 1);
        let addr15 = BlockAddr::new(0, 0, 0, 15);

        let buf0 = [0x1111u16; 20];
        let buf1 = [0x2222u16; 20];
        let buf15 = [0xFFFFu16; 20];

        disk.write_block20(addr0, &buf0).unwrap();
        disk.write_block20(addr1, &buf1).unwrap();
        disk.write_block20(addr15, &buf15).unwrap();

        // Read back and verify isolation
        let mut read0 = [0u16; 20];
        let mut read1 = [0u16; 20];
        let mut read15 = [0u16; 20];

        disk.read_block20(addr0, &mut read0).unwrap();
        disk.read_block20(addr1, &mut read1).unwrap();
        disk.read_block20(addr15, &mut read15).unwrap();

        assert_eq!(read0, buf0);
        assert_eq!(read1, buf1);
        assert_eq!(read15, buf15);
    }

    #[test]
    fn test_geometry_method_returns_correct_values() {
        let disk = Ibm2310::new(TimingModel::none());
        let geo = disk.geometry();

        assert_eq!(geo.cylinders, 200);
        assert_eq!(geo.heads, 2);
        assert_eq!(geo.sectors_per_track, 4);
        assert_eq!(geo.words_per_sector, 321);
    }
}
