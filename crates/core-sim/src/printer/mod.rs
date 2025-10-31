// IBM 1403 Line Printer

pub mod ibm1403;

use crate::disk::{Device, DeviceStatusWord, IoResult};

pub trait LinePrinter: Device {
    fn print_line(&mut self, line: &[u8]) -> IoResult<()>;
    fn get_output(&self) -> &[Vec<u8>];
    fn clear_output(&mut self);
}
