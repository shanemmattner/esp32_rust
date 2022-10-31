use esp_idf_hal::adc;
use esp_idf_hal::adc::PoweredAdc;
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::FromValueType;
use esp_idf_hal::{gpio, i2c};
use sx1509;

// Result<ST7789<SPIInterfaceNoCS<Master<SPI2, Gpio18<Unknown>, Gpio19<Unknown>, Gpio21<Unknown>, Gpio5<Unknown>>, Gpio16<Output>>, Gpio23<Output>>>
pub fn i2c_peripheral(
    per: Peripherals,
) -> i2c::Master<
    i2c::I2C0,
    esp_idf_hal::gpio::Gpio4<gpio::InputOutput>,
    esp_idf_hal::gpio::Gpio5<gpio::Output>,
> {
    let sda = per.pins.gpio4.into_input_output().unwrap();
    let scl = per.pins.gpio5.into_output().unwrap();
    let i2c = per.i2c0;

    let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());
    let i2c =
        i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config).unwrap();
    i2c
}

pub fn sx1509_init(
    i2c: &mut i2c::Master<i2c::I2C0, gpio::Gpio4<gpio::InputOutput>, gpio::Gpio5<gpio::Output>>,
) -> sx1509::Sx1509<i2c::Master<i2c::I2C0, gpio::Gpio4<gpio::InputOutput>, gpio::Gpio5<gpio::Output>>>
{
    let mut expander = sx1509::Sx1509::new(i2c, sx1509::DEFAULT_ADDRESS);
    // ptype(&expander);
    expander.borrow(i2c).software_reset().unwrap();
    expander.borrow(i2c).set_bank_a_direction(0).unwrap();
    expander.borrow(i2c).set_bank_b_direction(0xFF).unwrap();

    expander
}

// pub fn adc_init() -> PoweredAdc<adc::ADC1> {
//     let peripherals = Peripherals::take().unwrap();
//     let powered_adc1 = adc::PoweredAdc::new(
//         peripherals.adc1,
//         adc::config::Config::new().calibration(true),
//     )
//     .unwrap();

//     powered_adc1
// }
