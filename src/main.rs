#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use sam3x8e::{rtt, piob};

pub fn delay_ms(x: u32) {
    unsafe {
        let y = (*rtt()).vr.read() + x;
        // FIXME: deal with overflow!
        while (*rtt()).vr.read() < y {}
    }
}

#[entry]
fn main() -> ! {
    unsafe { (*piob()).per.write(1 << 27) }
    unsafe { (*piob()).oer.write(1 << 27) }
    unsafe { (*rtt()).mr.write(32768 / 1000) }
    let mut on = true;

    loop {
        delay_ms(250);
        if on {
            unsafe { (*piob()).codr.write(1 << 27) }
        } else {
            unsafe { (*piob()).sodr.write(1 << 27) }
        }
        on = !on;
    }
}

