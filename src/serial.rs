//! serial to USB based on PIO

use embassy_usb::class::cdc_acm;
use static_cell::StaticCell;
use embassy_executor::Spawner;
use embassy_usb::class::cdc_acm::{CdcAcmClass, Receiver, Sender, ControlChanged};
use embassy_rp::usb::Driver;
use embassy_rp::peripherals::USB;
use embassy_time::Timer;
use embassy_usb::driver::EndpointError;
use crate::board;

pub fn init(spawner: Spawner, usb_builder: &mut embassy_usb::Builder<'static, Driver<'static, USB>>) {
    static STATES: StaticCell<[cdc_acm::State; board::NUM_PORTS]> = StaticCell::new();
    let states = STATES.init([const { cdc_acm::State::new() }; board::NUM_PORTS]);

    for (port, state) in states.iter_mut().enumerate() {
        let cdc = CdcAcmClass::new(usb_builder, state, 64);
        let (usb_sender, usb_receiver, usb_control) = cdc.split_with_control();

        spawner.spawn(usb_to_uart(port, usb_receiver).unwrap());
        spawner.spawn(uart_to_usb(port, usb_sender).unwrap());
        spawner.spawn(uart_control(port, usb_control).unwrap());
    }
}

#[embassy_executor::task(pool_size = board::NUM_PORTS)]
async fn uart_to_usb(_port: usize, mut usb_sender: Sender<'static, Driver<'static, USB>>) {
    // TODO: We'll be DMAing data out of the uart into a buffer which this task follows
    // It might need to re-arm the DMA, TBD.

    loop {
        Timer::after_millis(500).await;
        match usb_sender.write_packet(b"tick\r\n").await {
            Ok(_) => (),
            Err(EndpointError::Disabled) => break,
            Err(EndpointError::BufferOverflow) => continue, // Shouldn't happen
        }
    }
}

#[embassy_executor::task(pool_size = board::NUM_PORTS)]
async fn usb_to_uart(_port: usize, mut usb_receiver: Receiver<'static, Driver<'static, USB>>) {
    // TODO: Instead of _port: usize, we need a serial port

    loop {
        usb_receiver.wait_connection().await;

        let mut buf = [0; 64];
        loop {
            let _data = match usb_receiver.read_packet(&mut buf).await {
                Ok(n) => &mut buf[..n],
                Err(EndpointError::Disabled) => break,
                Err(EndpointError::BufferOverflow) => continue, // Shouldn't happen
            };

            // TODO: We have _data with our incoming data. We should push it into the DMA buffer.

            // TODO: We don't actually want to push each byte directly, as we want to encode any
            // bits here - stop, parity, etc. And to support half-stop bits, we need to double up.
            // TODO: sm.tx().dma_push(_data)
        }
    }
}

#[embassy_executor::task(pool_size = crate::board::NUM_PORTS)]
async fn uart_control(_port: usize, usb_control: ControlChanged<'static>) {
   loop {
       usb_control.control_changed().await;

       // TODO: we need to update the PIO settings
       // The current release of embassy_usb doesn't expose line_settings here
       // so we need a workaround or something.

       // Or handle RTS/DTR, depending on what changed.
   }
}
