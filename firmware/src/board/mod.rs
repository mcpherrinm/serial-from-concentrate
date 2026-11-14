//! Based on a board feature flag, control what peripherals we have.

#[cfg(feature = "pi-pico")]
mod pi_pico;
#[cfg(feature = "pi-pico")]
pub use pi_pico::*;

#[cfg(feature = "pi-pico-2")]
mod pi_pico_2;
#[cfg(feature = "pi-pico-2")]
pub use pi_pico_2::*;

#[cfg(feature = "peripheral-board")]
mod peripheral_board;
#[cfg(feature = "peripheral-board")]
pub use peripheral_board::*;

#[cfg(not(any(feature = "pi-pico", feature = "pi-pico-2", feature = "peripheral-board")))]
compile_error!(
    "select a board feature: pi-pico, pi-pico-2, or peripheral-board (or use `cargo build-pico` etc.)"
);

#[cfg(any(
    all(feature = "pi-pico", feature = "pi-pico-2"),
    all(feature = "pi-pico", feature = "peripheral-board"),
    all(feature = "pi-pico-2", feature = "peripheral-board"),
))]
compile_error!("only one board feature may be enabled");
