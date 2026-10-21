use embassy_rp::bind_interrupts;
use static_cell::StaticCell;
use crate::board;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<embassy_rp::peripherals::USB>;
});

pub fn init(p: embassy_rp::Peri<'static, embassy_rp::peripherals::USB>) -> embassy_usb::Builder<'static, embassy_rp::usb::Driver<'static, embassy_rp::peripherals::USB>> {
    let driver = embassy_rp::usb::Driver::new(p, Irqs);
    let usb_config = {
        let mut config = embassy_usb::Config::new(board::VID, board::PID);
        config.manufacturer = Some(board::MFGR);
        config.product = Some(board::PRODUCT);
        config.serial_number = Some("1234"); // TODO: Once embassy-rp has a new release: Some(embassy_rp::uid::uid_hex());
        config.max_power = 100;
        config.max_packet_size_0 = 64;
        config
    };

    static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

    embassy_usb::Builder::new(
        driver,
        usb_config,
        CONFIG_DESCRIPTOR.init([0; 256]),
        BOS_DESCRIPTOR.init([0; 256]),
        &mut [], // no msos descriptors
        CONTROL_BUF.init([0; 64]),
    )
}
