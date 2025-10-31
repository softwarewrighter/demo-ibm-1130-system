//! Disk image file I/O operations.
//!
//! This module handles reading and writing .dsk disk image files for
//! IBM 1130 disk devices. The file format preserves all disk data including
//! geometry and sector contents.
//!
//! # File Format
//!
//! The .dsk file format consists of:
//! 1. 8-byte magic number: "I1130DSK"
//! 2. Geometry structure (cylinders, heads, sectors, words)
//! 3. 32 bytes reserved for future use
//! 4. Raw sector data (all cylinders/heads/sectors, 16-bit words, little-endian)
//!
//! Total header size: 48 bytes
//!
//! # Examples
//!
//! ```no_run
//! use core_sim::disk::file_io::{read_dsk_file, write_dsk_file};
//! use core_sim::disk::Geometry;
//! use std::path::Path;
//!
//! // Write a disk image
//! let geo = Geometry::IBM2315;
//! let data = vec![0u16; geo.total_words()];
//! write_dsk_file(Path::new("disk.dsk"), geo, &data).unwrap();
//!
//! // Read it back
//! let (geo_read, data_read) = read_dsk_file(Path::new("disk.dsk")).unwrap();
//! assert_eq!(geo_read, geo);
//! assert_eq!(data_read.len(), data.len());
//! ```

use super::{DskHeader, Geometry};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// Error type for disk file I/O operations.
#[derive(Debug, PartialEq, Eq)]
pub enum DskFileError {
    /// I/O error (file not found, permission denied, etc.)
    IoError(String),
    /// Invalid magic number (not "I1130DSK")
    InvalidMagic,
    /// Invalid geometry (unsupported or inconsistent values)
    InvalidGeometry,
    /// File size doesn't match expected size from geometry
    SizeMismatch { expected: usize, actual: usize },
    /// File is too short to contain a valid header
    FileTooShort,
}

impl From<io::Error> for DskFileError {
    fn from(err: io::Error) -> Self {
        DskFileError::IoError(err.to_string())
    }
}

impl std::fmt::Display for DskFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DskFileError::IoError(msg) => write!(f, "I/O error: {}", msg),
            DskFileError::InvalidMagic => write!(f, "Invalid magic number (not I1130DSK)"),
            DskFileError::InvalidGeometry => write!(f, "Invalid or unsupported geometry"),
            DskFileError::SizeMismatch { expected, actual } => {
                write!(
                    f,
                    "File size mismatch: expected {} bytes, got {}",
                    expected, actual
                )
            }
            DskFileError::FileTooShort => write!(f, "File too short for header"),
        }
    }
}

impl std::error::Error for DskFileError {}

/// Read a .dsk disk image file.
///
/// # Arguments
///
/// * `path` - Path to the .dsk file
///
/// # Returns
///
/// A tuple of (Geometry, Vec<u16>) containing the disk geometry and all sector data.
///
/// # Errors
///
/// Returns `DskFileError` if:
/// - File cannot be opened or read
/// - Magic number is invalid
/// - Geometry is invalid or unsupported
/// - File size doesn't match expected size
///
/// # Examples
///
/// ```no_run
/// use core_sim::disk::file_io::read_dsk_file;
/// use std::path::Path;
///
/// let (geo, data) = read_dsk_file(Path::new("test.dsk")).unwrap();
/// println!("Loaded disk: {} cylinders, {} total words", geo.cylinders, data.len());
/// ```
pub fn read_dsk_file(path: &Path) -> Result<(Geometry, Vec<u16>), DskFileError> {
    let mut file = File::open(path)?;

    // Read header (48 bytes)
    let mut header_bytes = [0u8; 48];
    file.read_exact(&mut header_bytes)?;

    // Parse magic number
    let magic = &header_bytes[0..8];
    if magic != DskHeader::MAGIC {
        return Err(DskFileError::InvalidMagic);
    }

    // Parse geometry (after magic, before reserved)
    let geo = parse_geometry(&header_bytes[8..16])?;

    // Calculate expected data size
    let expected_words = geo.total_words();
    let expected_bytes = expected_words * 2; // 16-bit words

    // Read all sector data
    let mut data_bytes = Vec::new();
    file.read_to_end(&mut data_bytes)?;

    if data_bytes.len() != expected_bytes {
        return Err(DskFileError::SizeMismatch {
            expected: expected_bytes,
            actual: data_bytes.len(),
        });
    }

    // Convert bytes to u16 words (little-endian)
    let data = bytes_to_words(&data_bytes);

    Ok((geo, data))
}

/// Write a .dsk disk image file.
///
/// # Arguments
///
/// * `path` - Path where the .dsk file will be created
/// * `geo` - Disk geometry
/// * `data` - All sector data (must match geometry.total_words() length)
///
/// # Errors
///
/// Returns `DskFileError` if:
/// - File cannot be created or written
/// - Data length doesn't match geometry
///
/// # Examples
///
/// ```no_run
/// use core_sim::disk::file_io::write_dsk_file;
/// use core_sim::disk::Geometry;
/// use std::path::Path;
///
/// let geo = Geometry::IBM2315;
/// let data = vec![0u16; geo.total_words()];
/// write_dsk_file(Path::new("blank.dsk"), geo, &data).unwrap();
/// ```
pub fn write_dsk_file(path: &Path, geo: Geometry, data: &[u16]) -> Result<(), DskFileError> {
    // Validate data length
    if data.len() != geo.total_words() {
        return Err(DskFileError::SizeMismatch {
            expected: geo.total_words(),
            actual: data.len(),
        });
    }

    let mut file = File::create(path)?;

    // Write header
    let header = DskHeader::new(geo);
    write_header(&mut file, &header)?;

    // Write sector data (little-endian)
    let data_bytes = words_to_bytes(data);
    file.write_all(&data_bytes)?;

    Ok(())
}

/// Parse geometry from 8 bytes of header data.
fn parse_geometry(bytes: &[u8]) -> Result<Geometry, DskFileError> {
    if bytes.len() < 8 {
        return Err(DskFileError::FileTooShort);
    }

    let cylinders = u16::from_le_bytes([bytes[0], bytes[1]]);
    let heads = bytes[2];
    let sectors_per_track = bytes[3];
    let words_per_sector = u16::from_le_bytes([bytes[4], bytes[5]]);

    // Validate geometry values
    if cylinders == 0 || heads == 0 || sectors_per_track == 0 || words_per_sector == 0 {
        return Err(DskFileError::InvalidGeometry);
    }

    Ok(Geometry {
        cylinders,
        heads,
        sectors_per_track,
        words_per_sector,
    })
}

/// Write header to file.
fn write_header(file: &mut File, header: &DskHeader) -> Result<(), DskFileError> {
    // Magic (8 bytes)
    file.write_all(&header.magic)?;

    // Geometry (8 bytes)
    file.write_all(&header.geo.cylinders.to_le_bytes())?;
    file.write_all(&[header.geo.heads])?;
    file.write_all(&[header.geo.sectors_per_track])?;
    file.write_all(&header.geo.words_per_sector.to_le_bytes())?;
    file.write_all(&[0, 0])?; // Padding to align

    // Reserved (32 bytes)
    file.write_all(&header.reserved)?;

    Ok(())
}

/// Convert little-endian bytes to u16 words.
fn bytes_to_words(bytes: &[u8]) -> Vec<u16> {
    bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect()
}

/// Convert u16 words to little-endian bytes.
fn words_to_bytes(words: &[u16]) -> Vec<u8> {
    words.iter().flat_map(|&word| word.to_le_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // ========== Helper Functions ==========

    fn create_test_geometry() -> Geometry {
        Geometry::IBM2315
    }

    fn create_test_data(geo: &Geometry) -> Vec<u16> {
        let mut data = vec![0u16; geo.total_words()];
        // Add some pattern for testing
        for i in 0..data.len().min(100) {
            data[i] = (i as u16).wrapping_mul(0x1111);
        }
        data
    }

    // ========== Conversion Tests ==========

    #[test]
    fn test_bytes_to_words_little_endian() {
        let bytes = vec![0x34, 0x12, 0x78, 0x56];
        let words = bytes_to_words(&bytes);
        assert_eq!(words, vec![0x1234, 0x5678]);
    }

    #[test]
    fn test_words_to_bytes_little_endian() {
        let words = vec![0x1234, 0x5678];
        let bytes = words_to_bytes(&words);
        assert_eq!(bytes, vec![0x34, 0x12, 0x78, 0x56]);
    }

    #[test]
    fn test_words_bytes_roundtrip() {
        let original_words = vec![0xABCD, 0x1234, 0x5678, 0xDEAD, 0xBEEF];
        let bytes = words_to_bytes(&original_words);
        let roundtrip_words = bytes_to_words(&bytes);
        assert_eq!(original_words, roundtrip_words);
    }

    // ========== Geometry Parsing Tests ==========

    #[test]
    fn test_parse_geometry_ibm2315() {
        let bytes = [
            200, 0, // cylinders (200)
            2, // heads
            4, // sectors_per_track
            0x41, 0x01, // words_per_sector (321)
            0, 0, // padding
        ];

        let geo = parse_geometry(&bytes).unwrap();
        assert_eq!(geo.cylinders, 200);
        assert_eq!(geo.heads, 2);
        assert_eq!(geo.sectors_per_track, 4);
        assert_eq!(geo.words_per_sector, 321);
    }

    #[test]
    fn test_parse_geometry_invalid_zero_cylinders() {
        let bytes = [0, 0, 2, 4, 0x41, 0x01, 0, 0];
        let result = parse_geometry(&bytes);
        assert_eq!(result, Err(DskFileError::InvalidGeometry));
    }

    #[test]
    fn test_parse_geometry_too_short() {
        let bytes = [200, 0, 2, 4]; // Only 4 bytes
        let result = parse_geometry(&bytes);
        assert_eq!(result, Err(DskFileError::FileTooShort));
    }

    // ========== File I/O Tests ==========

    #[test]
    fn test_write_dsk_file_creates_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.dsk");

        let geo = create_test_geometry();
        let data = create_test_data(&geo);

        let result = write_dsk_file(&file_path, geo, &data);
        assert!(result.is_ok());
        assert!(file_path.exists());
    }

    #[test]
    fn test_write_dsk_file_wrong_data_length() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.dsk");

        let geo = create_test_geometry();
        let wrong_data = vec![0u16; 1000]; // Wrong length

        let result = write_dsk_file(&file_path, geo, &wrong_data);
        assert!(result.is_err());
        match result {
            Err(DskFileError::SizeMismatch { expected, actual }) => {
                assert_eq!(expected, geo.total_words());
                assert_eq!(actual, 1000);
            }
            _ => panic!("Expected SizeMismatch error"),
        }
    }

    #[test]
    fn test_read_dsk_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.dsk");

        let geo = create_test_geometry();
        let original_data = create_test_data(&geo);

        // Write file
        write_dsk_file(&file_path, geo, &original_data).unwrap();

        // Read file back
        let (read_geo, read_data) = read_dsk_file(&file_path).unwrap();

        assert_eq!(read_geo, geo);
        assert_eq!(read_data.len(), original_data.len());
        assert_eq!(read_data, original_data);
    }

    #[test]
    fn test_read_dsk_file_nonexistent() {
        let result = read_dsk_file(Path::new("/nonexistent/path/test.dsk"));
        assert!(result.is_err());
        match result {
            Err(DskFileError::IoError(_)) => {} // Expected
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_read_dsk_file_invalid_magic() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.dsk");

        // Write file with wrong magic
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"BADMAGIC").unwrap(); // Wrong magic
        file.write_all(&[200, 0, 2, 4, 0x41, 0x01, 0, 0]).unwrap(); // Geometry
        file.write_all(&[0u8; 32]).unwrap(); // Reserved
        file.write_all(&vec![0u8; 1000]).unwrap(); // Some data

        let result = read_dsk_file(&file_path);
        assert_eq!(result, Err(DskFileError::InvalidMagic));
    }

    #[test]
    fn test_read_dsk_file_size_mismatch() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("short.dsk");

        let geo = create_test_geometry();
        let mut file = File::create(&file_path).unwrap();

        // Write valid header
        let header = DskHeader::new(geo);
        write_header(&mut file, &header).unwrap();

        // Write too little data
        file.write_all(&vec![0u8; 1000]).unwrap(); // Way too short

        let result = read_dsk_file(&file_path);
        assert!(result.is_err());
        match result {
            Err(DskFileError::SizeMismatch { .. }) => {} // Expected
            _ => panic!("Expected SizeMismatch error"),
        }
    }

    #[test]
    fn test_roundtrip_preserves_data_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("pattern.dsk");

        let geo = create_test_geometry();
        let mut original_data = vec![0u16; geo.total_words()];

        // Create interesting pattern
        for (i, word) in original_data.iter_mut().enumerate() {
            *word = ((i % 65536) as u16).wrapping_mul(0xABCD);
        }

        // Write and read
        write_dsk_file(&file_path, geo, &original_data).unwrap();
        let (_geo, read_data) = read_dsk_file(&file_path).unwrap();

        // Verify bit-for-bit identical
        assert_eq!(original_data, read_data);
    }

    #[test]
    fn test_roundtrip_all_bits_set() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("allbits.dsk");

        let geo = create_test_geometry();
        let original_data = vec![0xFFFFu16; geo.total_words()];

        write_dsk_file(&file_path, geo, &original_data).unwrap();
        let (_geo, read_data) = read_dsk_file(&file_path).unwrap();

        assert_eq!(original_data, read_data);
    }

    #[test]
    fn test_file_size_calculation() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("size.dsk");

        let geo = create_test_geometry();
        let data = vec![0u16; geo.total_words()];

        write_dsk_file(&file_path, geo, &data).unwrap();

        let metadata = fs::metadata(&file_path).unwrap();
        let expected_size = 48 + (geo.total_words() * 2); // Header + data
        assert_eq!(metadata.len(), expected_size as u64);
    }

    #[test]
    fn test_error_display() {
        let err = DskFileError::InvalidMagic;
        assert!(err.to_string().contains("magic"));

        let err = DskFileError::InvalidGeometry;
        assert!(err.to_string().contains("geometry"));

        let err = DskFileError::SizeMismatch {
            expected: 100,
            actual: 50,
        };
        assert!(err.to_string().contains("100"));
        assert!(err.to_string().contains("50"));
    }
}
