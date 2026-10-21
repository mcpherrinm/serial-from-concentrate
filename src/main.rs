//! Serial From Concentrate firmware, based on the blinky example.

#![no_std]
#![no_main]

mod board;
mod serial;
mod usb;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use panic_probe as _;

// Program metadata for `picotool info`.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"serial-from-concentrate"),
    embassy_rp::binary_info::rp_program_description!(c"Serial From Concentrate firmware"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main(executor = "embassy_rp::executor::Executor", entry = "cortex_m_rt::entry")]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut usb_builder = usb::init(p.USB);

    serial::init(spawner, &mut usb_builder);

    let mut led = Output::new(p.PIN_25, Level::Low);
    let blink = async {
        loop {
            led.set_high();
            Timer::after_millis(500).await;
            led.set_low();
            Timer::after_millis(500).await;
        }
    };

    let mut usb = usb_builder.build();

    join(usb.run(), blink).await;
}
