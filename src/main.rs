#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::SharedData;
use embassy_time::Timer;

use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

static SHARED_DATA: MaybeUninit<SharedData> = MaybeUninit::uninit();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init_primary(Default::default(), &SHARED_DATA);
    info!("Hello World!");

    let mut blue = Output::new(p.PB15, Level::Low, Speed::Low);
    let mut green = Output::new(p.PB9, Level::Low, Speed::Low);
    let mut red = Output::new(p.PB11, Level::Low, Speed::Low);

    loop {
        blue.set_high();
        Timer::after_millis(200).await;
        blue.set_low();
        green.set_high();
        Timer::after_millis(200).await;
        green.set_low();
        red.set_high();
        Timer::after_millis(200).await;
        red.set_low();
    }
}
