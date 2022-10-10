#![no_std]
#![no_main]

use core::fmt::Write;
use litex_hal::prelude::*;
use litex_sim_pac::{riscv_rt::entry, Peripherals};
use panic_halt as _;

litex_hal::uart! {
    Uart: litex_sim_pac::UART,
}

litex_hal::timer! {
    Timer: litex_sim_pac::TIMER0,
}

const SYSTEM_CLOCK_FREQUENCY: u32 = 1_000_000;

#[entry]
fn main() -> ! {
    let peripherals = unsafe { Peripherals::steal() };
    let mut uart = Uart::new(peripherals.UART);
    writeln!(uart, "Peripherals initialized").unwrap();

    let mut timer = Timer::new(peripherals.TIMER0, SYSTEM_CLOCK_FREQUENCY);
    let mut uptime = 0;
    loop {
        timer.delay_ms(1000_u32);
        uptime += 1;
        writeln!(uart, "Uptime: {} seconds", uptime).unwrap();
    }
}
