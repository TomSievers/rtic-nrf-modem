#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use nrf9160_hal as hal;
use panic_probe as _;
use tinyrlibc as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [PWM0])]
mod app {
    use super::*;

    use nrf_modem::SystemMode;
    use rtic_monotonics::{create_systick_token, systick::Systick};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let systick_token = create_systick_token!();
        Systick::start(cx.core.SYST, 64_000_000, systick_token);

        radio::spawn().ok();

        (Shared {}, Local {})
    }

    #[task(priority = 0)]
    async fn radio(_cx: radio::Context) {
        defmt::info!("Modem init");

        nrf_modem::init(SystemMode {
            lte_support: true,
            lte_psm_support: false,
            nbiot_support: true,
            gnss_support: false,
            preference: nrf_modem::ConnectionPreference::Lte,
        })
        .await
        .expect("Modem init failed");

        defmt::info!("Init done");
    }

    #[task(binds = IPC, priority = 7)]
    fn ipc(_cx: ipc::Context) {
        nrf_modem::ipc_irq_handler();
    }
}
