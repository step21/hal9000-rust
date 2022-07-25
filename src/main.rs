//! Blinks the LED on a Adafruit Feather RP2040 board
//!
//! This will blink on-board LED.
#![no_std]
#![no_main]

use oorandom;
use adafruit_feather_rp2040::entry;
use adafruit_feather_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use core::iter::once;
use embedded_hal::timer::CountDown;
use embedded_time::duration::Extensions;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::rate::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
//    let core = pac::CorePeripherals::take().unwrap();
    let seed = 10;
    let mut rng = oorandom::Rand32::new(seed);
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led_pin = pins.d13.into_push_pull_output();
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = timer.count_down();
    loop {
        led_pin.set_high().unwrap();
        let num1 = rng.rand_range(10..100).milliseconds();
        delay.start(num1);
        delay.wait();
        led_pin.set_low().unwrap();
        let num2 = rng.rand_range(10..100).milliseconds();
        delay.start(num2);
        delay.wait();
    }
}
