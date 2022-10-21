use esp_idf_hal::units::FromValueType;
use std::thread;
use std::time::Duration;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c;
use embedded_hal::i2c::blocking::I2c;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{config::MasterConfig, Master, MasterPins, I2C0},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

const SX1509_I2C_ADDR: u8 = 0x3E;

const SX1509_RegDataB: u8 = 0x10;
const SX1509_RegDataA: u8 = 0x11;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();

    // Instanciate the i2c peripheral, correct pins are in the training material.
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    // let i2c = init_i2c();
    let mut i2c = Master::<I2C0, _, _>::new(
        peripherals.i2c0,
        MasterPins { sda, scl },
        <MasterConfig as Default>::default().baudrate(400.kHz().into()),
    )
    .unwrap();

    let mut expander = sx1509::Sx1509::new(&mut i2c, sx1509::DEFAULT_ADDRESS);
    expander.borrow(&mut i2c).software_reset().unwrap();
    expander.borrow(&mut i2c).set_bank_a_direction(0).unwrap();
    loop {
        let pins = expander.borrow(&mut i2c).get_bank_a_data();
        // let mut buff: [u8; 1] = [0; 1];
        // i2c.write(SX1509_I2C_ADDR, &[0x11]).unwrap();
        // i2c.read(SX1509_I2C_ADDR, &mut buff).unwrap();
        log::info!("value is {:?}", pins);
        thread::sleep(Duration::from_millis(100));
    }
}

// fn init_i2c() -> I2C0 {
//     Master::<I2C0, _, _>::new(
//         peripherals.i2c0,
//         MasterPins { sda, scl },
//         <MasterConfig as Default>::default().baudrate(400.kHz().into()),
//     )?;
// }
