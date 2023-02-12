#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use cortex_m::asm;
// pick a panicking behavior
use {defmt_rtt as _, panic_probe as _}; // global logger

use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_executor::{Spawner};
use embassy_stm32::Config;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    let mut led = Output::new(p.PE12, Level::Low, Speed::Low).degrade();

    loop {
        asm::delay(8_000_000);
        led.toggle();
        asm::delay(8_000_000);
        led.toggle();
    }
}