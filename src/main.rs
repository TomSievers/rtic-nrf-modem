#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use nrf9160_hal as hal;
use panic_probe as _;
use defmt_rtt as _;
use fugit::ExtU32;
use hal::prelude::*;
use tinyrlibc as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [CRYPTOCELL])]
mod app {
    use super::*;

    use nrf_modem::SystemMode;
    use rtic_monotonics::{systick::Systick, create_systick_token};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let systick_token = create_systick_token!();
        Systick::start(cx.core.SYST, 64_000_000, systick_token);


        // Schedule the blinking task
        radio::spawn().ok();

        (Shared {}, Local { })
    }

    #[task(local = [])]
    async fn radio(_cx: radio::Context) {
        nrf_modem::init(SystemMode{
            lte_support: true,
            lte_psm_support: true,
            nbiot_support: true, 
            gnss_support: false,
            preference: nrf_modem::ConnectionPreference::Lte
        }).await.expect("init failed");

        loop {
            Systick::delay(1.secs()).await;
            defmt::info!("Print");
        }
    }
}