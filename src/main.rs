#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_itm as _;
use stm32l4 as _;
use stm32l4xx_hal::{
    delay::Delay, gpio::Output, gpio::Pin, gpio::PushPull, gpio::L8, pac, prelude::*,
};

#[entry]
fn main() -> ! {
    let (mut ld1, mut delay) = init();

    let half_period = 500_u16;

    loop {
        ld1.toggle();
        delay.delay_ms(half_period);
    }
}

// The following would, most likely, be placed in a different module.
type Led1 = Pin<Output<PushPull>, L8, 'A', 5>;

fn init() -> (Led1, Delay) {
    // Take ownership of the stm32l475 cortex m and device peripherals.
    let device_periphs = pac::Peripherals::take().unwrap();
    let core_periphs = cortex_m::Peripherals::take().unwrap();

    // Constrain and configure the RCC (clock control) peripheral.
    let mut rcc = device_periphs.RCC.constrain();
    let mut flash = device_periphs.FLASH.constrain();
    let mut pwr = device_periphs.PWR.constrain(&mut rcc.apb1r1);

    // Set up the system clock and freeze it (finalize configuration).
    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    // Acquire GPIOA and configure PA5 as a push-pull output.
    let mut gpioa = device_periphs.GPIOA.split(&mut rcc.ahb2);
    let ld1 = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let delay = Delay::new(core_periphs.SYST, clocks);

    (ld1, delay)
}
