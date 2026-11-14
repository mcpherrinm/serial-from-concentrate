//! Peripheral board with a single RP2350b and 6 RS232 transcievers

pub const VID: u16 = 0x1209; // pid.codes VID
pub const PID: u16 = 0x000A; // pid.codes Test PID
pub const MFGR: &str = "TBD";
pub const PRODUCT: &str = "Serial Peripheral Board";

// NUM_PORTS is how many serial ports this board has
pub const NUM_PORTS: usize = 6;
