//! serial to USB based on PIO

use embassy_usb::class::cdc_acm;
use static_cell::StaticCell;
use embassy_executor::Spawner;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_rp::usb::Driver;
use embassy_rp::peripherals::USB;
use embassy_usb::driver::EndpointError;
use crate::board;

pub fn init(spawner: Spawner, usb_builder: &mut embassy_usb::Builder<'static, Driver<'static, USB>>) {
    static STATES: StaticCell<[cdc_acm::State; board::NUM_PORTS]> = StaticCell::new();
    let states = STATES.init([const { cdc_acm::State::new() }; board::NUM_PORTS]);

    for (port, state) in states.iter_mut().enumerate() {
        let cdc = CdcAcmClass::new(usb_builder, state, 64);

        spawner.spawn(task(port, cdc).unwrap());
    }

}

#[embassy_executor::task(pool_size = crate::board::NUM_PORTS)]
async fn task(port: usize, mut acm: CdcAcmClass<'static, Driver<'static, USB>>) {
    loop {
        let mut buf = [0; 64];
        acm.wait_connection().await;

       match acm.write_packet(b"Hello from port ").await {
           Err(EndpointError::BufferOverflow) => defmt::panic!("Buffer overflow"),
           Err(EndpointError::Disabled) => break,
           Ok(_) => (),
       }

        let portn = match port {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            _ => "higher",
        };

        match acm.write_packet(portn.as_ref()).await {
            Err(EndpointError::BufferOverflow) => defmt::panic!("Buffer overflow"),
            Err(EndpointError::Disabled) => break,
            Ok(_) => (),
        }

        loop {
            let data: &mut [u8] = match acm.read_packet(&mut buf).await {
                Err(EndpointError::BufferOverflow) => defmt::panic!("Buffer overflow"),
                Err(EndpointError::Disabled) => break,
                Ok(n) => &mut buf[..n],
            };

            match acm.write_packet(data).await {
                Err(EndpointError::BufferOverflow) => defmt::panic!("Buffer overflow"),
                Err(EndpointError::Disabled) => break,
                Ok(_) => (),
            }
        }
    }
}
