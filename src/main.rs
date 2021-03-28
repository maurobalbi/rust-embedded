#![no_std]
#![no_main]

use hal::{gpio::GpioExt, prelude::{InputPin, OutputPin}, rcc::RccExt};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32l4xx_hal as hal;

#[entry]
fn main() -> ! {
    let mut peripherals = hal::stm32::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.constrain();

    let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb2);
    let mut gpioc = peripherals.GPIOC.split(&mut rcc.ahb2);

    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut btn = gpioc.pc13.into_pull_up_input(&mut gpioc.moder, &mut gpioc.pupdr);

    let ms = 500;
    loop {
    let bit = btn.is_low().unwrap();

        if bit {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
