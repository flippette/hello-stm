#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::{main, task, Spawner};
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    peripherals::PA5,
    time::Hertz,
    Config,
};
use embassy_time::{Duration, Timer};

#[main]
async fn main(spawner: Spawner) {
    let mut cfg = Config::default();
    cfg.rcc.sys_ck = Some(Hertz(84_000_000));
    let dp = embassy_stm32::init(cfg);

    let led = Output::new(dp.PA5, Level::Low, Speed::Low);

    spawner.spawn(hello()).unwrap();
    spawner.spawn(blink_led(led)).unwrap();
}

#[task]
async fn hello() {
    loop {
        Timer::after(Duration::from_secs(1)).await;
        info!("hello, world!");
    }
}

#[task]
async fn blink_led(mut led: Output<'static, PA5>) {
    loop {
        Timer::after(Duration::from_millis(500)).await;
        led.toggle();
        info!(
            "{}",
            match led.is_set_high() {
                true => "on!",
                false => "off!",
            }
        );
    }
}
