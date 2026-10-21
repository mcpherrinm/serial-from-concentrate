//! Development on a Pi Pico (RP2040)

pub const VID: u16 = 0x1209; // pid.codes VID
pub const PID: u16 = 0x000A; // pid.codes Test PID
pub const MFGR: &str = "Raspberry Pi";
pub const PRODUCT: &str = "RP2040 Pi Pico";

// NUM_PORTS is how many serial ports this board has
pub const NUM_PORTS: usize = 2;

// some way of mapping UARTs to pins...
