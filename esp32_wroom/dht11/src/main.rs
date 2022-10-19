// Code copied from https://github.com/snasirca/rust-esp32-c3-blinky/blob/main/src/main.rs
use std::thread;
use std::time::Duration;

// use embedded_hal::digital::v2::IoPin;
use embedded_hal::digital::v2::OutputPin;

use esp_idf_hal::delay;
// use esp_idf_hal::gpio;

use esp_idf_hal::peripherals::Peripherals;

use dht11::Dht11;

fn main() {
    esp_idf_sys::link_patches();
    sensible_env_logger::init!();

    let peripherals = Peripherals::take().unwrap();

    let mut led = peripherals.pins.gpio2.into_output().unwrap();

    let dht11_pin = peripherals.pins.gpio23.into_input();
    let mut dht11 = Dht11::new(dht11_pin);

    loop {
        println!("Toggle");
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(1000));

        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(1000));

        let mut delay = delay::Ets;
        let measurement = dht11.perform_measurement(&mut delay).unwrap();
        println!("{:?}", measurement);
    }
}
