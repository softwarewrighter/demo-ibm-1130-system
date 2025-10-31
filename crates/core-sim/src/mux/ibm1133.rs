// IBM 1133 Multiplexor for device attachment routing

use super::*;
use std::collections::HashMap;

pub struct Ibm1133 {
    devices: HashMap<u8, Box<dyn Device>>,
    status: DeviceStatusWord,
}

impl Ibm1133 {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            status: DeviceStatusWord {
                busy: false,
                error: false,
                attention: false,
            },
        }
    }
}

impl Default for Ibm1133 {
    fn default() -> Self {
        Self::new()
    }
}

impl Device for Ibm1133 {
    fn reset(&mut self) {
        for dev in self.devices.values_mut() {
            dev.reset();
        }
        self.status = DeviceStatusWord {
            busy: false,
            error: false,
            attention: false,
        };
    }

    fn poll(&mut self, now_us: u64) {
        for dev in self.devices.values_mut() {
            dev.poll(now_us);
        }
    }

    fn dsw(&self) -> DeviceStatusWord {
        self.status
    }
}

impl Multiplexor for Ibm1133 {
    fn attach(&mut self, dev: Box<dyn Device>, dev_code: u8) {
        self.devices.insert(dev_code, dev);
    }

    fn issue_iocc(&mut self, dev_code: u8, _cmd: IoCommand) -> IoResult<()> {
        // TODO: implement IOCC command routing
        if self.devices.contains_key(&dev_code) {
            Ok(())
        } else {
            Err(IoError::InvalidAddress)
        }
    }
}
