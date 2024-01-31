#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use nrf9160_hal as hal;
use panic_probe as _;
use tinyrlibc as _;
use fugit::ExtU32;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [PWM0])]
mod app {
    use super::*;

    use nrf_modem::{SystemMode, no_std_net::IpAddr};
    use rtic_monotonics::{create_systick_token, systick::Systick};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let systick_token = create_systick_token!();
        Systick::start(cx.core.SYST, 64_000_000, systick_token);

        failing_radio::spawn().ok();

        (Shared {}, Local {})
    }

    #[task(priority = 1)]
    async fn failing_radio(_cx: failing_radio::Context) {
        defmt::info!("Modem init");

        let res = nrf_modem::init(SystemMode {
            lte_support: true,
            lte_psm_support: false,
            nbiot_support: true,
            gnss_support: false,
            preference: nrf_modem::ConnectionPreference::Lte,
        })
        .await;
        

        defmt::info!("Init done {:?}", res);

        succes_radio::spawn().ok();
    }

    #[task(priority = 0)]
    async fn succes_radio(_cx: succes_radio::Context) {
        defmt::info!("Modem init");

        let res = nrf_modem::init(SystemMode {
            lte_support: true,
            lte_psm_support: false,
            nbiot_support: true,
            gnss_support: false,
            preference: nrf_modem::ConnectionPreference::Lte,
        })
        .await;
        

        defmt::info!("Init done {:?}", res);

        send::spawn().ok();
    }

    #[task(priority = 1)]
    async fn send(_cx: send::Context) {
        loop {
            let self_ip : IpAddr = "0.0.0.0".parse().unwrap();
            let sock = nrf_modem::UdpSocket::bind((self_ip, 53)).await.expect("Failed to get socket");

            let server_ip : IpAddr = "10.60.8.90".parse().unwrap();

            sock.send_to(b"Hello World", (server_ip, 4445).into()).await.expect("Failed to send");

            sock.deactivate().await.ok();

            defmt::info!("Sent");

            Systick::delay(120.secs()).await;
        }
    }

    #[task(binds = IPC, priority = 7)]
    fn ipc(_cx: ipc::Context) {
        nrf_modem::ipc_irq_handler();
    }
}
