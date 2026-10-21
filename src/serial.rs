//! serial to USB based on PIO

use embassy_usb::class::cdc_acm;
use static_cell::StaticCell;
use embassy_executor::Spawner;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_rp::usb::Driver;
use embassy_rp::peripherals::USB;
use embassy_usb::driver::EndpointError;

pub fn init(spawner: Spawner, usb_builder: &mut embassy_usb::Builder<'static, Driver<'static, USB>>) {
    static STATE: StaticCell<cdc_acm::State> = StaticCell::new();
    let state = STATE.init(cdc_acm::State::new());

    let cdc = CdcAcmClass::new(usb_builder, state, 64);

    spawner.spawn(task(cdc).unwrap());
}

#[embassy_executor::task]
async fn task(mut acm: CdcAcmClass<'static, Driver<'static, USB>>) {
    loop {
        let mut buf = [0; 64];
        acm.wait_connection().await;

        loop {
            let data: &mut [u8] = match acm.read_packet(&mut buf).await {
                Err(EndpointError::BufferOverflow) => defmt::panic!("Buffer overflow"),
                Err(EndpointError::Disabled) => return,
                Ok(n) => &mut buf[..n],
            };

            match acm.write_packet(data).await {
                Err(EndpointError::BufferOverflow) => defmt::panic!("Buffer overflow"),
                Err(EndpointError::Disabled) => return,
                Ok(_) => (),
            }
        }
    }
}
