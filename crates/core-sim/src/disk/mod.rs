// IBM 1130 Disk device implementations

pub mod ibm2310;
pub mod ibm2311;

use crate::timing::TimingModel;

#[derive(Clone, Copy, Debug)]
pub struct Geometry {
    pub cylinders: u16,        // 200 logical, 203 physical with 3 alternates for 2315
    pub heads: u8,             // 2 (top/bottom)
    pub sectors_per_track: u8, // 4
    pub words_per_sector: u16, // 321 (word 0 = sector address)
}

impl Geometry {
    /// IBM 2315 cartridge / 2310 single drive geometry
    pub const IBM2315: Self = Self {
        cylinders: 200,
        heads: 2,
        sectors_per_track: 4,
        words_per_sector: 321,
    };

    /// IBM 2311 Model 11 geometry (1316 disk pack)
    pub const IBM2311_MODEL11: Self = Self {
        cylinders: 203, // TODO: verify actual cylinder count
        heads: 10,
        sectors_per_track: 4,
        words_per_sector: 321,
    };

    /// IBM 2311 Model 12 geometry
    pub const IBM2311_MODEL12: Self = Self {
        cylinders: 203, // TODO: verify actual cylinder count
        heads: 6,
        sectors_per_track: 4,
        words_per_sector: 321,
    };

    pub fn total_sectors(&self) -> usize {
        self.cylinders as usize * self.heads as usize * self.sectors_per_track as usize
    }

    pub fn total_words(&self) -> usize {
        self.total_sectors() * self.words_per_sector as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BlockAddr {
    pub cyl: u16,
    pub head: u8,   // 0 or 1 for 2315
    pub sector: u8, // 0..=7 (0..=3 top, 4..=7 bottom)
    pub block: u8,  // 0..=15 within sector (20 words each)
}

impl BlockAddr {
    pub fn new(cyl: u16, head: u8, sector: u8, block: u8) -> Self {
        Self {
            cyl,
            head,
            sector,
            block,
        }
    }

    /// Convert to linear sector index
    pub fn to_sector_index(&self, geo: &Geometry) -> usize {
        let sectors_per_cyl = geo.heads as usize * geo.sectors_per_track as usize;
        let cyl_offset = self.cyl as usize * sectors_per_cyl;
        let head_offset = self.head as usize * geo.sectors_per_track as usize;
        cyl_offset + head_offset + self.sector as usize
    }

    /// Calculate word offset within sector (skipping address word)
    pub fn word_offset_in_sector(&self) -> usize {
        1 + (self.block as usize * 20)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SeekOutcome {
    pub target_cyl: u16,
    pub quantized_cyl: u16, // 2315 seeks in increments of 2
    pub time_us: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoError {
    SeekError,
    ReadError,
    WriteError,
    ParityError,
    InvalidAddress,
    DeviceBusy,
}

pub type IoResult<T> = Result<T, IoError>;

#[derive(Debug, Clone, Copy)]
pub struct DeviceStatusWord {
    pub busy: bool,
    pub error: bool,
    pub attention: bool,
}

pub trait Device {
    fn reset(&mut self);
    fn poll(&mut self, now_us: u64);
    fn dsw(&self) -> DeviceStatusWord;
}

pub trait DiskDevice: Device {
    fn geometry(&self) -> Geometry;
    fn seek(&mut self, cyl: u16) -> SeekOutcome;
    fn select_head(&mut self, head: u8);
    fn read_sector(&mut self, cyl: u16, head: u8, sector: u8, buf: &mut [u16; 321])
    -> IoResult<()>;
    fn write_sector(&mut self, cyl: u16, head: u8, sector: u8, buf: &[u16; 321]) -> IoResult<()>;
    fn read_block20(&mut self, addr: BlockAddr, buf: &mut [u16; 20]) -> IoResult<()>;
    fn write_block20(&mut self, addr: BlockAddr, buf: &[u16; 20]) -> IoResult<()>;
}

/// Disk image file format header
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DskHeader {
    pub magic: [u8; 8], // "I1130DSK"
    pub geo: Geometry,
    pub reserved: [u8; 32],
}

impl DskHeader {
    pub const MAGIC: &'static [u8; 8] = b"I1130DSK";

    pub fn new(geo: Geometry) -> Self {
        Self {
            magic: *Self::MAGIC,
            geo,
            reserved: [0; 32],
        }
    }

    pub fn validate(&self) -> bool {
        &self.magic == Self::MAGIC
    }
}
