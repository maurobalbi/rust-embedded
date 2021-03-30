#![no_std]
#![no_main]

use hal::{
    gpio::{gpioa::PA5, gpioc::PC13, ExtiPin, GpioExt, Input, Output, PullUp, PushPull},
    interrupt,
    prelude::OutputPin,
    rcc::RccExt,
};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::{borrow::Borrow, cell::RefCell, ops::DerefMut};
use cortex_m::{
    interrupt::{free, Mutex},
    peripheral::NVIC,
};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32l4xx_hal as hal;

static BUTTON: Mutex<RefCell<Option<PC13<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
static LED: Mutex<RefCell<Option<PA5<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut peripherals = hal::stm32::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.constrain();

    let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb2);
    let mut gpioc = peripherals.GPIOC.split(&mut rcc.ahb2);

    let mut led = gpioa.pa5.into_push_pull_output_with_state(
        &mut gpioa.moder,
        &mut gpioa.otyper,
        hal::gpio::State::Low,
    );

    let mut btn = gpioc
        .pc13
        .into_pull_up_input(&mut gpioc.moder, &mut gpioc.pupdr);

    btn.make_interrupt_source(&mut peripherals.SYSCFG, &mut rcc.apb2);
    btn.enable_interrupt(&mut peripherals.EXTI);
    btn.trigger_on_edge(&mut peripherals.EXTI, hal::gpio::Edge::FALLING);

    unsafe { NVIC::unmask(hal::stm32::Interrupt::EXTI15_10) }

    free(|cs| {
        BUTTON.borrow(cs).replace(Some(btn));
        LED.borrow(cs).replace(Some(led))
    });

    loop {
        continue;
    }
}

#[interrupt]
fn EXTI15_10() {
    free(|cs| {
        let mut led_ref = LED.borrow(cs).borrow_mut();
        let mut btn_ref = BUTTON.borrow(cs).borrow_mut();
        if let Some(ref mut btn) = btn_ref.deref_mut() {
            if btn.check_interrupt() {
                btn.clear_interrupt_pending_bit();
            }
        };
        if let Some(ref mut led) = led_ref.deref_mut() {
            let low = unsafe { (*hal::stm32::GPIOA::ptr()).odr.read().bits() & (1 << 5) == 0 };

            if low {
                led.set_high().unwrap();
            } else {
                led.set_low().unwrap();
            }
        }
    })
}
