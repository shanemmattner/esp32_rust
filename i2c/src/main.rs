use embedded_hal::blocking::i2c::{Read, Write};
use esp_idf_hal::i2c;
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::FromValueType;
use esp_idf_sys as _;
use std::thread;
use std::time::Duration;

const SX1509_I2C_ADDR: u8 = 0x3E;

const SX1509_RegDataB: u8 = 0x10;
const SX1509_RegDataA: u8 = 0x11;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let per = Peripherals::take().unwrap();
    let sda = per.pins.gpio4.into_input_output().unwrap();
    let scl = per.pins.gpio5.into_output().unwrap();
    let i2c = per.i2c0;

    let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());
    let mut i2cdev =
        i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config).unwrap();

    loop {
        let mut buff: [u8; 1] = [0; 1];
        i2cdev.write(SX1509_I2C_ADDR, &[0x11]).unwrap();
        i2cdev.read(SX1509_I2C_ADDR, &mut buff).unwrap();
        log::info!("wai value is {:?}", buff);
        thread::sleep(Duration::from_millis(100));
    }
}
