// Create sample .dsk files for testing and demonstration
//
// Run with: cargo run --example create_sample_disks

use core_sim::disk::{file_io, Geometry};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating sample IBM 1130 disk images...\n");

    // 1. Blank disk
    create_blank_disk()?;

    // 2. Disk with data in first few sectors
    create_test_data_disk()?;

    // 3. Disk with pattern data
    create_pattern_disk()?;

    println!("\n✓ All sample disk images created successfully!");
    println!("  Location: crates/fixtures/data/disks/");

    Ok(())
}

fn create_blank_disk() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. Creating blank_disk.dsk...");

    let geo = Geometry::IBM2315;
    let data = vec![0u16; geo.total_words()];

    file_io::write_dsk_file(Path::new("crates/fixtures/data/disks/blank_disk.dsk"), geo, &data)?;

    println!("   ✓ Created: 512,000 words, all zeros");
    Ok(())
}

fn create_test_data_disk() -> Result<(), Box<dyn std::error::Error>> {
    println!("2. Creating test_data.dsk...");

    let geo = Geometry::IBM2315;
    let mut data = vec![0u16; geo.total_words()];

    // Write some test data to first few sectors
    // Cylinder 0, Head 0, Sectors 0-3
    let words_per_sector = 321;

    // Sector 0: Sequential numbers
    for i in 0..320 {
        data[i] = i as u16;
    }

    // Sector 1: Powers of 2
    let sector1_offset = words_per_sector;
    for i in 0..320 {
        data[sector1_offset + i] = 1u16 << (i % 16);
    }

    // Sector 2: Alternating pattern
    let sector2_offset = words_per_sector * 2;
    for i in 0..320 {
        data[sector2_offset + i] = if i % 2 == 0 { 0xAAAA } else { 0x5555 };
    }

    // Sector 3: ASCII text simulation
    let sector3_offset = words_per_sector * 3;
    let text = "IBM 1130 DISK STORAGE SYSTEM - TEST DATA";
    for (i, ch) in text.chars().enumerate() {
        if i < 320 {
            data[sector3_offset + i] = ch as u16;
        }
    }

    // Add some data on cylinder 10, head 0
    let cyl10_offset = 10 * 8 * words_per_sector; // 8 sectors per cylinder
    for i in 0..100 {
        data[cyl10_offset + i] = 0x1000 + i as u16;
    }

    file_io::write_dsk_file(Path::new("crates/fixtures/data/disks/test_data.dsk"), geo, &data)?;

    println!("   ✓ Created: Test data in cylinders 0 and 10");
    Ok(())
}

fn create_pattern_disk() -> Result<(), Box<dyn std::error::Error>> {
    println!("3. Creating pattern_disk.dsk...");

    let geo = Geometry::IBM2315;
    let mut data = vec![0u16; geo.total_words()];

    // Create a checkerboard pattern across the entire disk
    // Every other word in every other sector has data
    let words_per_sector = 321;
    let sectors_per_track = 4;
    let tracks_per_cylinder = 2;

    for cyl in 0..geo.cylinders {
        for head in 0..tracks_per_cylinder {
            for sector in 0..sectors_per_track {
                // Calculate base offset for this sector
                let sector_index = (cyl as usize * tracks_per_cylinder as usize * sectors_per_track as usize)
                    + (head as usize * sectors_per_track as usize)
                    + sector as usize;

                let base_offset = sector_index * words_per_sector;

                // Checkerboard: only write to even sectors
                if sector % 2 == 0 {
                    // Write pattern data
                    for i in 0..320 {
                        if i % 2 == 0 {
                            // Encode cylinder and head in the data
                            data[base_offset + i] = ((cyl as u16) << 8) | (head as u16);
                        }
                    }
                }
            }
        }
    }

    file_io::write_dsk_file(
        Path::new("crates/fixtures/data/disks/pattern_disk.dsk"),
        geo,
        &data,
    )?;

    println!("   ✓ Created: Checkerboard pattern across all cylinders");
    Ok(())
}
