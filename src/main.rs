#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32l4::stm32l4x6;
use stm32l4::stm32l4x6::{tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Set the timer to go off in `ms` ticks
    // 1 tick = 1 ms
    tim6.arr.write(|w| w.arr().bits(ms));

    // CEN: Enable the counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());

    // Wait until the alarm goes off (until the update event occurs)
    while !tim6.sr.read().uif().bit_is_set() {}

    // Clear the update event flag
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}


#[entry]
fn main() -> ! {
    let peripherals = stm32l4x6::Peripherals::take().unwrap();

    let tim6 = &peripherals.TIM6;

    let rcc = &peripherals.RCC;

    rcc.ahb2enr.modify(|_, w| w.gpioaen().set_bit());
    rcc.apb1enr1.modify(|_, w| w.tim6en().set_bit());
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    tim6.psc.write(|w| w.psc().bits(7_999));

    let gpioa = &peripherals.GPIOA;

    gpioa.moder.modify(|_, w| w.moder5().output());
    
    let ms = 500;
    loop {
        gpioa.odr.modify(|_, w| w.odr5().set_bit());
        delay(tim6, ms);
        gpioa.odr.modify(|_, w| w.odr5().clear_bit());
        delay(tim6, ms)
    }
}
