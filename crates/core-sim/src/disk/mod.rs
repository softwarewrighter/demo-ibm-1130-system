//! IBM 1130 disk device implementations.
//!
//! This module provides traits and types for simulating IBM 1130 disk drives,
//! including the IBM 2310 (2315 cartridge) and IBM 2311 (1316 disk pack).

pub mod ibm2310;
pub mod ibm2311;

use crate::timing::TimingModel;

/// Disk geometry specification.
///
/// Defines the physical characteristics of an IBM 1130 disk drive:
/// - Cylinders: Concentric tracks at different radii
/// - Heads: Read/write heads (one per surface)
/// - Sectors per track: Addressable segments of each track
/// - Words per sector: 16-bit words in each sector (includes address word)
///
/// # Examples
///
/// ```
/// use core_sim::disk::Geometry;
///
/// let geo = Geometry::IBM2315;
/// assert_eq!(geo.cylinders, 200);
/// assert_eq!(geo.heads, 2);
/// assert_eq!(geo.total_sectors(), 1600); // 200 * 2 * 4
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Geometry {
    /// Number of cylinders (200 logical for 2315, 203 physical with 3 alternates)
    pub cylinders: u16,
    /// Number of heads (2 for 2315: top/bottom surfaces)
    pub heads: u8,
    /// Sectors per track (4 for IBM 1130 disks)
    pub sectors_per_track: u8,
    /// Words per sector (321: word 0 = sector address, 320 data words)
    pub words_per_sector: u16,
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

/// Logical block address for Disk Monitor System (DMS).
///
/// DMS addresses data in 20-word blocks within sectors. Each sector
/// contains 16 logical blocks (blocks 0-15).
///
/// # IBM 2315 Sector Numbering
///
/// Sectors are numbered 0-7 across both heads:
/// - Sectors 0-3: Head 0 (top surface)
/// - Sectors 4-7: Head 1 (bottom surface)
///
/// # Examples
///
/// ```
/// use core_sim::disk::{BlockAddr, Geometry};
///
/// let addr = BlockAddr::new(50, 1, 2, 5);
/// assert_eq!(addr.cyl, 50);
/// assert_eq!(addr.head, 1);
/// assert_eq!(addr.sector, 2);
/// assert_eq!(addr.block, 5);
///
/// // Calculate linear sector index
/// let geo = Geometry::IBM2315;
/// let index = addr.to_sector_index(&geo);
/// assert_eq!(index, 50 * 8 + 1 * 4 + 2); // 406
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockAddr {
    /// Cylinder number (0..199 for 2315)
    pub cyl: u16,
    /// Head number (0 or 1 for 2315)
    pub head: u8,
    /// Sector number (0..=7 for 2315: 0..=3 head 0, 4..=7 head 1)
    pub sector: u8,
    /// Block number within sector (0..=15, each block is 20 words)
    pub block: u8,
}

impl BlockAddr {
    /// Create a new block address.
    ///
    /// # Arguments
    ///
    /// * `cyl` - Cylinder number
    /// * `head` - Head number (0 or 1 for 2315)
    /// * `sector` - Sector number (0..=7 for 2315)
    /// * `block` - Block number within sector (0..=15)
    ///
    /// # Examples
    ///
    /// ```
    /// use core_sim::BlockAddr;
    ///
    /// let addr = BlockAddr::new(100, 0, 3, 7);
    /// assert_eq!(addr.word_offset_in_sector(), 141); // 1 + 7*20
    /// ```
    pub fn new(cyl: u16, head: u8, sector: u8, block: u8) -> Self {
        Self {
            cyl,
            head,
            sector,
            block,
        }
    }

    /// Convert to linear sector index.
    ///
    /// Calculates the absolute sector number in disk image files.
    ///
    /// Formula: `cyl * sectors_per_cyl + head * sectors_per_track + sector`
    ///
    /// # Arguments
    ///
    /// * `geo` - Disk geometry specification
    ///
    /// # Examples
    ///
    /// ```
    /// use core_sim::disk::{BlockAddr, Geometry};
    ///
    /// let addr = BlockAddr::new(0, 0, 0, 0);
    /// let geo = Geometry::IBM2315;
    /// assert_eq!(addr.to_sector_index(&geo), 0);
    ///
    /// let addr = BlockAddr::new(0, 1, 0, 0);
    /// assert_eq!(addr.to_sector_index(&geo), 4); // Head 1 offset
    /// ```
    pub fn to_sector_index(&self, geo: &Geometry) -> usize {
        let sectors_per_cyl = geo.heads as usize * geo.sectors_per_track as usize;
        let cyl_offset = self.cyl as usize * sectors_per_cyl;
        let head_offset = self.head as usize * geo.sectors_per_track as usize;
        cyl_offset + head_offset + self.sector as usize
    }

    /// Calculate word offset within sector (skipping sector address word).
    ///
    /// The first word (offset 0) of each sector is the sector address word.
    /// Block data starts at offset 1.
    ///
    /// Formula: `1 + block * 20`
    ///
    /// # Examples
    ///
    /// ```
    /// use core_sim::BlockAddr;
    ///
    /// let addr = BlockAddr::new(0, 0, 0, 0);
    /// assert_eq!(addr.word_offset_in_sector(), 1); // First block starts at word 1
    ///
    /// let addr = BlockAddr::new(0, 0, 0, 15);
    /// assert_eq!(addr.word_offset_in_sector(), 301); // Last block: 1 + 15*20
    /// ```
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

/// Device Status Word (DSW) representing device state.
///
/// Matches IBM 1130 I/O channel status conventions with bit layout:
/// - Bit 15: busy (device performing operation)
/// - Bit 14: error (error occurred)
/// - Bit 13: attention (device needs service)
/// - Bit 12: ready (device ready for command)
///
/// # Examples
///
/// ```
/// use core_sim::DeviceStatusWord;
///
/// let dsw = DeviceStatusWord {
///     busy: false,
///     error: false,
///     attention: false,
///     ready: true,
/// };
///
/// assert_eq!(dsw.to_u16(), 0x1000); // Bit 12 set
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceStatusWord {
    /// Device is performing an operation
    pub busy: bool,
    /// Error occurred during operation
    pub error: bool,
    /// Device needs service or attention
    pub attention: bool,
    /// Device is ready for commands
    pub ready: bool,
}

impl DeviceStatusWord {
    /// Create a new DSW with all flags cleared.
    pub fn new() -> Self {
        Self {
            busy: false,
            error: false,
            attention: false,
            ready: false,
        }
    }

    /// Create a DSW indicating device is ready.
    pub fn ready() -> Self {
        Self {
            busy: false,
            error: false,
            attention: false,
            ready: true,
        }
    }

    /// Create a DSW indicating device is busy.
    pub fn busy() -> Self {
        Self {
            busy: true,
            error: false,
            attention: false,
            ready: false,
        }
    }

    /// Convert DSW to 16-bit word matching IBM 1130 format.
    ///
    /// Bit layout:
    /// - Bit 15 (0x8000): busy
    /// - Bit 14 (0x4000): error
    /// - Bit 13 (0x2000): attention
    /// - Bit 12 (0x1000): ready
    ///
    /// # Examples
    ///
    /// ```
    /// use core_sim::DeviceStatusWord;
    ///
    /// let dsw = DeviceStatusWord {
    ///     busy: true,
    ///     error: false,
    ///     attention: false,
    ///     ready: false,
    /// };
    ///
    /// assert_eq!(dsw.to_u16(), 0x8000);
    /// ```
    pub fn to_u16(&self) -> u16 {
        let mut word = 0u16;
        if self.busy {
            word |= 0x8000;
        } // Bit 15
        if self.error {
            word |= 0x4000;
        } // Bit 14
        if self.attention {
            word |= 0x2000;
        } // Bit 13
        if self.ready {
            word |= 0x1000;
        } // Bit 12
        word
    }

    /// Create DSW from 16-bit word.
    ///
    /// # Examples
    ///
    /// ```
    /// use core_sim::DeviceStatusWord;
    ///
    /// let dsw = DeviceStatusWord::from_u16(0x8000);
    /// assert!(dsw.busy);
    /// assert!(!dsw.error);
    /// assert!(!dsw.attention);
    /// assert!(!dsw.ready);
    /// ```
    pub fn from_u16(word: u16) -> Self {
        Self {
            busy: (word & 0x8000) != 0,
            error: (word & 0x4000) != 0,
            attention: (word & 0x2000) != 0,
            ready: (word & 0x1000) != 0,
        }
    }
}

impl Default for DeviceStatusWord {
    fn default() -> Self {
        Self::new()
    }
}

/// Base trait for all IBM 1130 devices.
///
/// All devices (disk, card reader, printer, etc.) implement this trait
/// to provide uniform lifecycle management and status reporting.
///
/// # Lifecycle
///
/// 1. Device is created via factory function (e.g., `make_2315()`)
/// 2. `reset()` initializes device to power-on state
/// 3. Operations are performed (seek, read, write, etc.)
/// 4. `poll()` is called periodically to advance timers
/// 5. `dsw()` reports current device status
///
/// # Examples
///
/// ```
/// use core_sim::disk::{Device, DeviceStatusWord};
///
/// // Implementors must provide these methods
/// struct MyDevice;
/// impl Device for MyDevice {
///     fn reset(&mut self) { /* Initialize device */ }
///     fn poll(&mut self, now_us: u64) { /* Update timers */ }
///     fn dsw(&self) -> DeviceStatusWord {
///         DeviceStatusWord::ready()
///     }
/// }
/// ```
pub trait Device {
    /// Reset device to power-on state.
    ///
    /// Clears all errors, cancels pending operations, and initializes
    /// device to ready state.
    fn reset(&mut self);

    /// Advance device timers and complete pending operations.
    ///
    /// Should be called periodically (e.g., every frame in UI loop) with
    /// monotonically increasing microsecond timestamps.
    ///
    /// # Arguments
    ///
    /// * `now_us` - Current time in microseconds since epoch
    fn poll(&mut self, now_us: u64);

    /// Get current Device Status Word (DSW).
    ///
    /// Returns status flags indicating device state (busy, error, attention, ready).
    fn dsw(&self) -> DeviceStatusWord;
}

/// Trait for IBM 1130 disk devices (2310, 2311).
///
/// Provides disk-specific operations including seek, read, and write
/// at both sector and logical block granularity.
///
/// # Addressing Modes
///
/// - **Cylinder/Head/Sector (CHS)**: Physical disk addressing
/// - **Block**: Logical 20-word blocks for DMS file system
///
/// # Examples
///
/// ```no_run
/// use core_sim::disk::{DiskDevice, Device, BlockAddr, Geometry};
/// use core_sim::api::make_2315;
/// use core_sim::TimingModel;
///
/// let mut disk = make_2315(TimingModel::none());
/// disk.reset();
///
/// // Seek to cylinder 100
/// let outcome = disk.seek(100);
/// assert_eq!(outcome.quantized_cyl, 100);
///
/// // Read a 20-word block
/// let addr = BlockAddr::new(100, 0, 0, 0);
/// let mut buf = [0u16; 20];
/// disk.read_block20(addr, &mut buf);
/// ```
pub trait DiskDevice: Device {
    /// Get disk geometry specification.
    fn geometry(&self) -> Geometry;

    /// Seek to specified cylinder.
    ///
    /// Initiates head movement to target cylinder. For 2315, seeks are
    /// quantized to 2-cylinder increments.
    ///
    /// # Arguments
    ///
    /// * `cyl` - Target cylinder number
    ///
    /// # Returns
    ///
    /// `SeekOutcome` containing target, quantized cylinder, and seek time
    fn seek(&mut self, cyl: u16) -> SeekOutcome;

    /// Select active read/write head.
    ///
    /// # Arguments
    ///
    /// * `head` - Head number (0 or 1 for 2315)
    fn select_head(&mut self, head: u8);

    /// Read entire sector (321 words).
    ///
    /// Reads sector address word (word 0) plus 320 data words.
    ///
    /// # Arguments
    ///
    /// * `cyl` - Cylinder number
    /// * `head` - Head number
    /// * `sector` - Sector number (0..=7 for 2315)
    /// * `buf` - Buffer to receive 321 words
    ///
    /// # Errors
    ///
    /// Returns `IoError` if address is invalid or device is busy.
    fn read_sector(&mut self, cyl: u16, head: u8, sector: u8, buf: &mut [u16; 321])
    -> IoResult<()>;

    /// Write entire sector (321 words).
    ///
    /// Writes sector address word (word 0) plus 320 data words.
    ///
    /// # Arguments
    ///
    /// * `cyl` - Cylinder number
    /// * `head` - Head number
    /// * `sector` - Sector number (0..=7 for 2315)
    /// * `buf` - Buffer containing 321 words to write
    ///
    /// # Errors
    ///
    /// Returns `IoError` if address is invalid or device is busy.
    fn write_sector(&mut self, cyl: u16, head: u8, sector: u8, buf: &[u16; 321]) -> IoResult<()>;

    /// Read logical 20-word block.
    ///
    /// Used by DMS file system for block-oriented I/O.
    ///
    /// # Arguments
    ///
    /// * `addr` - Block address (cyl/head/sector/block)
    /// * `buf` - Buffer to receive 20 words
    ///
    /// # Errors
    ///
    /// Returns `IoError` if address is invalid or device is busy.
    fn read_block20(&mut self, addr: BlockAddr, buf: &mut [u16; 20]) -> IoResult<()>;

    /// Write logical 20-word block.
    ///
    /// Used by DMS file system for block-oriented I/O.
    ///
    /// # Arguments
    ///
    /// * `addr` - Block address (cyl/head/sector/block)
    /// * `buf` - Buffer containing 20 words to write
    ///
    /// # Errors
    ///
    /// Returns `IoError` if address is invalid or device is busy.
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

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Geometry Tests ==========

    #[test]
    fn test_geometry_ibm2315_constants() {
        let geo = Geometry::IBM2315;
        assert_eq!(geo.cylinders, 200);
        assert_eq!(geo.heads, 2);
        assert_eq!(geo.sectors_per_track, 4);
        assert_eq!(geo.words_per_sector, 321);
    }

    #[test]
    fn test_geometry_total_sectors() {
        let geo = Geometry::IBM2315;
        // 200 cylinders * 2 heads * 4 sectors = 1600 sectors
        assert_eq!(geo.total_sectors(), 1600);
    }

    #[test]
    fn test_geometry_total_words() {
        let geo = Geometry::IBM2315;
        // 1600 sectors * 321 words = 513,600 words
        assert_eq!(geo.total_words(), 513_600);
    }

    #[test]
    fn test_geometry_equality() {
        let geo1 = Geometry::IBM2315;
        let geo2 = Geometry {
            cylinders: 200,
            heads: 2,
            sectors_per_track: 4,
            words_per_sector: 321,
        };
        assert_eq!(geo1, geo2);
    }

    // ========== BlockAddr Tests ==========

    #[test]
    fn test_blockaddr_new() {
        let addr = BlockAddr::new(50, 1, 2, 5);
        assert_eq!(addr.cyl, 50);
        assert_eq!(addr.head, 1);
        assert_eq!(addr.sector, 2);
        assert_eq!(addr.block, 5);
    }

    #[test]
    fn test_blockaddr_to_sector_index_first_sector() {
        let geo = Geometry::IBM2315;
        let addr = BlockAddr::new(0, 0, 0, 0);
        assert_eq!(addr.to_sector_index(&geo), 0);
    }

    #[test]
    fn test_blockaddr_to_sector_index_head_offset() {
        let geo = Geometry::IBM2315;
        // Head 1 starts at sector 4 (sectors_per_track = 4)
        let addr = BlockAddr::new(0, 1, 0, 0);
        assert_eq!(addr.to_sector_index(&geo), 4);
    }

    #[test]
    fn test_blockaddr_to_sector_index_cylinder_offset() {
        let geo = Geometry::IBM2315;
        // Cyl 1 starts at sector 8 (2 heads * 4 sectors = 8)
        let addr = BlockAddr::new(1, 0, 0, 0);
        assert_eq!(addr.to_sector_index(&geo), 8);
    }

    #[test]
    fn test_blockaddr_to_sector_index_complex() {
        let geo = Geometry::IBM2315;
        // Cyl 50, Head 1, Sector 2
        // = 50 * 8 + 1 * 4 + 2 = 406
        let addr = BlockAddr::new(50, 1, 2, 0);
        assert_eq!(addr.to_sector_index(&geo), 406);
    }

    #[test]
    fn test_blockaddr_word_offset_first_block() {
        let addr = BlockAddr::new(0, 0, 0, 0);
        // First block starts at word 1 (word 0 is sector address)
        assert_eq!(addr.word_offset_in_sector(), 1);
    }

    #[test]
    fn test_blockaddr_word_offset_last_block() {
        let addr = BlockAddr::new(0, 0, 0, 15);
        // Block 15 starts at word 1 + 15*20 = 301
        assert_eq!(addr.word_offset_in_sector(), 301);
    }

    #[test]
    fn test_blockaddr_word_offset_middle_block() {
        let addr = BlockAddr::new(0, 0, 0, 7);
        // Block 7 starts at word 1 + 7*20 = 141
        assert_eq!(addr.word_offset_in_sector(), 141);
    }

    // ========== DeviceStatusWord Tests ==========

    #[test]
    fn test_dsw_new_all_clear() {
        let dsw = DeviceStatusWord::new();
        assert!(!dsw.busy);
        assert!(!dsw.error);
        assert!(!dsw.attention);
        assert!(!dsw.ready);
    }

    #[test]
    fn test_dsw_ready_constructor() {
        let dsw = DeviceStatusWord::ready();
        assert!(!dsw.busy);
        assert!(!dsw.error);
        assert!(!dsw.attention);
        assert!(dsw.ready);
    }

    #[test]
    fn test_dsw_busy_constructor() {
        let dsw = DeviceStatusWord::busy();
        assert!(dsw.busy);
        assert!(!dsw.error);
        assert!(!dsw.attention);
        assert!(!dsw.ready);
    }

    #[test]
    fn test_dsw_to_u16_all_clear() {
        let dsw = DeviceStatusWord::new();
        assert_eq!(dsw.to_u16(), 0x0000);
    }

    #[test]
    fn test_dsw_to_u16_busy_only() {
        let dsw = DeviceStatusWord {
            busy: true,
            error: false,
            attention: false,
            ready: false,
        };
        assert_eq!(dsw.to_u16(), 0x8000); // Bit 15
    }

    #[test]
    fn test_dsw_to_u16_error_only() {
        let dsw = DeviceStatusWord {
            busy: false,
            error: true,
            attention: false,
            ready: false,
        };
        assert_eq!(dsw.to_u16(), 0x4000); // Bit 14
    }

    #[test]
    fn test_dsw_to_u16_attention_only() {
        let dsw = DeviceStatusWord {
            busy: false,
            error: false,
            attention: true,
            ready: false,
        };
        assert_eq!(dsw.to_u16(), 0x2000); // Bit 13
    }

    #[test]
    fn test_dsw_to_u16_ready_only() {
        let dsw = DeviceStatusWord {
            busy: false,
            error: false,
            attention: false,
            ready: true,
        };
        assert_eq!(dsw.to_u16(), 0x1000); // Bit 12
    }

    #[test]
    fn test_dsw_to_u16_all_set() {
        let dsw = DeviceStatusWord {
            busy: true,
            error: true,
            attention: true,
            ready: true,
        };
        assert_eq!(dsw.to_u16(), 0xF000); // Bits 15-12
    }

    #[test]
    fn test_dsw_from_u16_all_clear() {
        let dsw = DeviceStatusWord::from_u16(0x0000);
        assert!(!dsw.busy);
        assert!(!dsw.error);
        assert!(!dsw.attention);
        assert!(!dsw.ready);
    }

    #[test]
    fn test_dsw_from_u16_busy() {
        let dsw = DeviceStatusWord::from_u16(0x8000);
        assert!(dsw.busy);
        assert!(!dsw.error);
        assert!(!dsw.attention);
        assert!(!dsw.ready);
    }

    #[test]
    fn test_dsw_from_u16_all_set() {
        let dsw = DeviceStatusWord::from_u16(0xF000);
        assert!(dsw.busy);
        assert!(dsw.error);
        assert!(dsw.attention);
        assert!(dsw.ready);
    }

    #[test]
    fn test_dsw_roundtrip() {
        let original = DeviceStatusWord {
            busy: true,
            error: false,
            attention: true,
            ready: false,
        };
        let word = original.to_u16();
        let roundtrip = DeviceStatusWord::from_u16(word);
        assert_eq!(original, roundtrip);
    }

    #[test]
    fn test_dsw_default_is_new() {
        let dsw = DeviceStatusWord::default();
        assert_eq!(dsw, DeviceStatusWord::new());
    }

    // ========== IoError Tests ==========

    #[test]
    fn test_ioerror_variants_exist() {
        // Just verify all variants compile
        let _errors = [
            IoError::SeekError,
            IoError::ReadError,
            IoError::WriteError,
            IoError::ParityError,
            IoError::InvalidAddress,
            IoError::DeviceBusy,
        ];
    }

    #[test]
    fn test_ioerror_equality() {
        assert_eq!(IoError::SeekError, IoError::SeekError);
        assert_ne!(IoError::SeekError, IoError::ReadError);
    }

    // ========== DskHeader Tests ==========

    #[test]
    fn test_dskheader_new() {
        let geo = Geometry::IBM2315;
        let header = DskHeader::new(geo);
        assert_eq!(&header.magic, b"I1130DSK");
        assert_eq!(header.geo, geo);
        assert_eq!(header.reserved, [0u8; 32]);
    }

    #[test]
    fn test_dskheader_validate_valid() {
        let geo = Geometry::IBM2315;
        let header = DskHeader::new(geo);
        assert!(header.validate());
    }

    #[test]
    fn test_dskheader_validate_invalid() {
        let mut header = DskHeader::new(Geometry::IBM2315);
        header.magic = *b"INVALID!";
        assert!(!header.validate());
    }
}
