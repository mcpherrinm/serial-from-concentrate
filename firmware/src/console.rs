use embassy_executor::Spawner;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_usb::class::cdc_acm;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use static_cell::StaticCell;

pub fn init(spawner: Spawner, usb_builder: &mut embassy_usb::Builder<'static, Driver<'static, USB>>) {
    static STATE: StaticCell<cdc_acm::State> = StaticCell::new();
    let state = STATE.init(cdc_acm::State::new());

    spawner.spawn(console(CdcAcmClass::new(usb_builder, state, 64)).unwrap());
}

#[embassy_executor::task]
async fn console(mut cdc: CdcAcmClass<'static, Driver<'static, USB>>) {
    loop {
        cdc.wait_connection().await;

        let mut buf = [0; 64];
        loop {
            let data = match cdc.read_packet(&mut buf).await {
                Ok(n) => &mut buf[..n],
                Err(_) => break,
            };

            // This is just a silly placeholder. We'll split the actual console logic out from the
            // USB handling, enabling it to be run off a UART or outside of hardware.
            if data.contains(&b'\n') || data.contains(&b'\r') {
                if cdc.write_packet(b"\r\n").await.is_err()
                    || cdc.write_packet(cli::hello().as_bytes()).await.is_err()
                    || cdc.write_packet(b"\r\n >").await.is_err()
                {
                    break;
                }
            }

           match cdc.write_packet(data).await {
               Ok(()) => (),
               Err(_) => break,
           }
        }
    }
}
