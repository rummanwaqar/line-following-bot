#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take()
    ) {
        // set up the LED (PC13)
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        // set up the system clock
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(100.mhz()).freeze();
        
        // create a delay abstraction based on system tick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            led.set_high().unwrap();
            delay.delay_ms(1000_u32);
            led.set_low().unwrap();
            delay.delay_ms(1000_u32);
            hprintln!("hey there").unwrap();
        }
    }
    loop {}
}
