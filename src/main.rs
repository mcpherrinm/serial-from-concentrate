//! Serial From Concentrate firmware, based on the blinky example.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, gpio};
use embassy_time::Timer;
use gpio::{Level, Output};
use panic_probe as _;
use static_cell::StaticCell;
use embassy_usb::class::cdc_acm;
use embassy_usb::driver::EndpointError;

// Program metadata for `picotool info`.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"serial-from-concentrate"),
    embassy_rp::binary_info::rp_program_description!(c"Serial From Concentrate firmware"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<embassy_rp::peripherals::USB>;
});

const VID: u16 = 0x1209; // pid.codes VID
const PID: u16 = 0x000A; // pid.codes Test PID

#[embassy_executor::main(executor = "embassy_rp::executor::Executor", entry = "cortex_m_rt::entry")]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_25, Level::Low);
    led.set_high();
    Timer::after_millis(100).await;
    led.set_low();
    Timer::after_millis(100).await;

    let driver = embassy_rp::usb::Driver::new(p.USB, Irqs);
    let usb_config = {
        let mut config = embassy_usb::Config::new(VID, PID);
        config.manufacturer = Some("Serial From Concentrate");
        config.product = Some("RP2040 USB-Serial");
        config.serial_number = Some("1"); // Get from OTP?
        config.max_power = 100;
        config.max_packet_size_0 = 64;
        config
    };

    let mut builder = {
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

        let builder = embassy_usb::Builder::new(
            driver,
            usb_config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            &mut [], // no msos descriptors
            CONTROL_BUF.init([0; 64]),
        );
        builder
    };

    let mut class = {
        static STATE: StaticCell<cdc_acm::State> = StaticCell::new();
        let state = STATE.init(cdc_acm::State::new());

        cdc_acm::CdcAcmClass::new(&mut builder, state, 64)
    };

    let mut usb = builder.build();

    let echo = async {
        let mut buf = [0; 64];
        class.wait_connection().await;
        loop {
            let data: &[u8] = match class.read_packet(&mut buf).await {
                Err(EndpointError::BufferOverflow) => defmt::panic!("Buffer overflow"),
                Err(EndpointError::Disabled) => return,
                Ok(n) => &buf[..n],
            };

            info!("connected!");
            led.set_high();

            match class.write_packet(data).await {
                Err(EndpointError::BufferOverflow) => defmt::panic!("Buffer overflow"),
                Err(EndpointError::Disabled) => return,
                Ok(_) => (),
            }

            Timer::after_millis(100).await;
            led.set_low();
            Timer::after_millis(100).await;
        }
    };

    embassy_futures::join::join(echo, usb.run()).await;
}
