pub mod audio;
pub mod card;
pub mod cpu_bus;
pub mod disk;
pub mod mux;
pub mod printer;
pub mod timing;
pub mod util;

// Re-export commonly used types
pub use audio::{AudioModel, SeekProfile};
pub use card::{Card80, CardDevice, CardStatus};
pub use disk::{BlockAddr, Device, DeviceStatusWord, DiskDevice, Geometry, IoError, IoResult};
pub use mux::Multiplexor;
pub use printer::LinePrinter;
pub use timing::TimingModel;

// Factory functions for creating devices
pub mod api {
    use super::*;
    use card::ibm1442::Ibm1442;
    use disk::{ibm2310::Ibm2310, ibm2311::Ibm2311};
    use mux::ibm1133::Ibm1133;
    use printer::ibm1403::Ibm1403;

    pub fn make_2315(timing: TimingModel) -> Ibm2310 {
        Ibm2310::new(timing)
    }

    pub fn make_2311(model_12: bool, timing: TimingModel) -> Ibm2311 {
        Ibm2311::new(model_12, timing)
    }

    pub fn make_1442(timing: TimingModel) -> Ibm1442 {
        Ibm1442::new(timing)
    }

    pub fn make_1403(model_7: bool, timing: TimingModel) -> Ibm1403 {
        Ibm1403::new(model_7, timing)
    }

    pub fn make_1133() -> Ibm1133 {
        Ibm1133::new()
    }
}
