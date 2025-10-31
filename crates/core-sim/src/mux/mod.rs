// IBM 1133 Multiplexor

pub mod ibm1133;

use crate::disk::{Device, DeviceStatusWord, IoError, IoResult};

#[derive(Debug, Clone, Copy)]
pub struct IoCommand {
    pub cmd_code: u8,
    pub modifier: u8,
}

pub trait Multiplexor: Device {
    fn attach(&mut self, dev: Box<dyn Device>, dev_code: u8);
    fn issue_iocc(&mut self, dev_code: u8, cmd: IoCommand) -> IoResult<()>;
}
