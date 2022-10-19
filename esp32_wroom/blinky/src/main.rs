// code adapted from https://github.com/snasirca/rust-esp32-c3-blinky/blob/main/src/main.rs
use std::thread;
use std::time::Duration;

use embedded_hal::digital::blocking::OutputPin;
use esp_idf_hal::peripherals::Peripherals;
use oorandom;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio2.into_output().unwrap();

    led.set_high().unwrap();
    thread::sleep(Duration::from_millis(200));

    led.set_low().unwrap();
    thread::sleep(Duration::from_millis(200));

    let mut counter: u64 = 0;
    loop {
        // let num = counter;
        let some_seed = counter;
        let mut rng = oorandom::Rand32::new(some_seed);
        let num = rng.rand_i32() / 1000;

        println!("{} {} ", counter, num);

        counter += 1;
        // led.set_high().unwrap();
        thread::sleep(Duration::from_millis(100));

        // led.set_low().unwrap();
        // thread::sleep(Duration::from_millis(5));
    }
}
