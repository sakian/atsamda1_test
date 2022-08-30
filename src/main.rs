#![no_std]
#![no_main]

use panic_probe as _;

use atsamd_hal as _;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Started");
    loop {}
}
