#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use atsamd_hal as hal;
use cortex_m_rt::entry;
use hal::bsp_pins;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

bsp_pins!(
    PA04 {
        name: system_led
        aliases: {
            PushPullOutput: SystemLedOut
        }
    },
    PA14 {
        name: xosc_in
        aliases: {
            FloatingDisabled: XoscIn
        }
    },
    PA15 {
        name: xosc_out
        aliases: {
            FloatingDisabled: XoscOut
        }
    },
    PA22 {
        name: host_txd
        aliases: {
            AlternateD: HostUartTxd
        }
    },
    PA23 {
        name: host_rxd
        aliases: {
            AlternateD: HostUartRxd
        }
    },
    PA24 {
        name: host_txe
        aliases: {
            PushPullOutput: HostTxeOut
        }
    },
);

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = Pins::new(peripherals.PORT);
    let mut led: SystemLedOut = pins.system_led.into();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    defmt::info!("Started");
    loop {
        delay.delay_ms(200u8);
        led.set_high().unwrap();
        delay.delay_ms(200u8);
        led.set_low().unwrap();
    }
}
