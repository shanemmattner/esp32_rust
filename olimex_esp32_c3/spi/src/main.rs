//! SPI loopback test
//!
//! Folowing pins are used:
//! SCLK   GPIO6
//! SDI    GPIO2
//! SDO    GPIO7
//! CS     GPIO10
//!
//! Depending on your target and the board you are using you have to change the pins.
//!
//! This example transfers data via SPI.
//! Connect SDI and SDO pins to see the outgoing data is read as incoming data.

use std::thread;
use std::time::Duration;

use embedded_hal::spi::SpiDevice;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::*;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let spi: SPI2 = peripherals.spi2;

    // Use SPI2, HSPI

    let sclk = peripherals.pins.gpio10;
    let serial_in = peripherals.pins.gpio8; // SDI / MISO?
    let serial_out = peripherals.pins.gpio7; // SDO / MOSI?
    let cs = peripherals.pins.gpio9;

    println!("Starting SPI loopback test");
    let config = config::Config::new().baudrate(26.MHz().into());
    let mut spidev = spi::SpiMasterDriver::<SPI2>::new(
        spi,
        sclk,
        serial_out,
        Some(serial_in),
        Some(cs),
        &config,
    );

    let mut read = [0u8; 4];
    let write = [0xde, 0xad, 0xbe, 0xef];

    loop {
        // we are using thread::sleep here to make sure the watchdog isn't triggered
        thread::sleep(Duration::from_millis(500));
        spidev.transfer(&mut read, &write);
        spidev.read(&mut read);
        println!("Wrote {:x?}, read {:x?}", write, read);
    }
}
