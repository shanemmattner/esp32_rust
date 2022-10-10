// code adapted from https://github.com/snasirca/rust-esp32-c3-blinky/blob/main/src/main.rs
use std::thread;
use std::time::Duration;

use embedded_hal::digital::blocking::OutputPin;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio2.into_output().unwrap();

    loop {
        println!("Toggle");
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(1000));

        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}
