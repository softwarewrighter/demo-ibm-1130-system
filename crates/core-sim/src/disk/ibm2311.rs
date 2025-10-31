// IBM 2311 Disk Storage Drive (uses 1316 disk pack)

use super::*;

pub struct Ibm2311 {
    geometry: Geometry,
    timing: TimingModel,
    current_cyl: u16,
    current_head: u8,
    status: DeviceStatusWord,
    data: Vec<u16>,
}

impl Ibm2311 {
    pub fn new(model_12: bool, timing: TimingModel) -> Self {
        let geometry = if model_12 {
            Geometry::IBM2311_MODEL12
        } else {
            Geometry::IBM2311_MODEL11
        };
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

impl Device for Ibm2311 {
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

impl DiskDevice for Ibm2311 {
    fn geometry(&self) -> Geometry {
        self.geometry
    }

    fn seek(&mut self, cyl: u16) -> SeekOutcome {
        // 2311 has different seek characteristics than 2315
        // TODO: implement accurate 2311 seek timing
        let time_us = self.timing.delay_us(50_000); // placeholder: 50ms average
        self.current_cyl = cyl;

        SeekOutcome {
            target_cyl: cyl,
            quantized_cyl: cyl,
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

        let _delay = self.timing.delay_us(20_000); // avg 20ms rotational latency

        Ok(())
    }

    fn write_sector(&mut self, cyl: u16, head: u8, sector: u8, buf: &[u16; 321]) -> IoResult<()> {
        let offset = self
            .sector_to_offset(cyl, head, sector)
            .ok_or(IoError::InvalidAddress)?;

        self.data[offset..offset + 321].copy_from_slice(buf);

        let _delay = self.timing.delay_us(8_900);

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
