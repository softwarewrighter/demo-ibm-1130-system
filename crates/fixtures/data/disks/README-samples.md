# Sample IBM 1130 Disk Images

Sample .dsk files for testing and demonstration.

## Files

- **blank_disk.dsk** - Empty disk (all zeros)
- **test_data.dsk** - Test data on cylinders 0 and 10
- **pattern_disk.dsk** - Checkerboard pattern across entire disk

## Usage

Load these files in the web UI to see different sector patterns:
- Gray sectors = free (no data)
- Blue sectors = used (contains data)

Generate with: `cargo run --example create_sample_disks`
