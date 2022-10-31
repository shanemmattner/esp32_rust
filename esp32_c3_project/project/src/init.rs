use crate::helpers;
use esp_idf_hal::i2c;
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::FromValueType;
use sx1509;

// Result<ST7789<SPIInterfaceNoCS<Master<SPI2, Gpio18<Unknown>, Gpio19<Unknown>, Gpio21<Unknown>, Gpio5<Unknown>>, Gpio16<Output>>, Gpio23<Output>>>
pub fn i2c_peripheral(
    per: Peripherals,
) -> i2c::Master<
    i2c::I2C0,
    esp_idf_hal::gpio::Gpio4<esp_idf_hal::gpio::InputOutput>,
    esp_idf_hal::gpio::Gpio5<esp_idf_hal::gpio::Output>,
> {
    // init::i2c_peripheral(&per);
    let sda = per.pins.gpio4.into_input_output().unwrap();
    let scl = per.pins.gpio5.into_output().unwrap();
    let i2c = per.i2c0;

    let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());
    let i2c =
        i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config).unwrap();
    i2c
}
