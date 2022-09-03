#![no_std]
#![no_main]

use cortex_m::Peripherals as CorePeripherals;
use cortex_m_semihosting::hprintln;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32f446_hal::{
    delay::Delay,
    prelude::{_embedded_hal_blocking_delay_DelayMs, _stm32f429_hal_flash_FlashExt, _stm32f429_hal_gpio_GpioExt, _embedded_hal_digital_OutputPin},
    rcc::RccExt,
    stm32f446::{Peripherals},
};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb1);
 
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut delay = Delay::new(cp.SYST, clocks);

    let delay_time: u32 = 1000;

    loop {
        led.set_low();

        delay.delay_ms(delay_time);

        led.set_high();

        delay.delay_ms(delay_time);
    }
}
