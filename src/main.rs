#![no_std]
#![no_main]

extern crate panic_halt;
extern crate sam3x;

use cortex_m_rt::entry;
use sam3x::hal::peripherals::{Peripheral};
use sam3x::hal::rtt::{init_timer, wait_ms};
use sam3x::drivers::led::{Led};

#[entry]
fn main() -> ! {
    init_timer();
    let led = Led::connect(Peripheral::PioB, 27).expect("illegal led pin");
    let mut on = true;

    loop {
        wait_ms(50);
        if on {
            led.off()
        } else {
            led.on()
        }
        on = !on;
    }
}

