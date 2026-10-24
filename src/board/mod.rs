//! Based on a board feature flag, control what peripherals we have.

#[cfg(feature = "pi-pico")]
mod pi_pico;
#[cfg(feature = "pi-pico")]
pub use pi_pico::*;

#[cfg(feature = "peripheral-board")]
mod peripheral_board;
#[cfg(feature = "peripheral-board")]
pub use peripheral_board::*;
