# Hello world

**The goal of this first step is to blink the LED and send strings to user over semi-hosting.**

*Follows [blinky example](https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/delay-blinky.rs) from [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal)*

## Dependencies

* `cortex-m`: micro-architecture crate for cortex-m MCUs; routines and peripherals common to all micro-controllers using a 
particular processor core
* `cortex-m-rt`: startup code and minimal runtime for cortex-m MCUs
* `stm32f4xx-hal`: HAL for STM32F4 MCUs; use features = ["rt", "stm32f411"]
* `cortex-m-semihosting`: implementation for communication over semihosting
* `panic-halt`: specifies panic behaviour (halt)

## Memory file

`cortex-m-rt` crate expects users to specify a memory.x file that specifies FLASH and
 RAM memory regions.

```
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K 
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
```

## .cargo/config file

It specifices the build target for cross-compilation

```
[target.thumbv7em-none-eabihf]
rustflags = [
  "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv7em-none-eabihf"
```

## Code

```C++
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
            hprintln!("Hello world!").unwrap();
        }
    }
    loop {}
}

```