use embedded_hal::digital::v2::OutputPin;
use std::thread;
use std::time::Duration;

use super::init;

pub fn thread1() {
    thread::spawn(|| {
        let mut i = 0;
        while true {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1000));
            i += 1;
        }
    });
}

pub fn blinky(b: &mut init::Board) {
    thread::spawn(|| {
        let mut i = 0;
        while true {
            // b.led.set_low().unwrap();
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(100));
            i += 1;
        }
    });
}
