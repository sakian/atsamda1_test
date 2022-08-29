#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use atsamd_hal as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    defmt::info!("Started");
    loop {}
}
