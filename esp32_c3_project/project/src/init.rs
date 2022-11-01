use esp_idf_hal::adc::PoweredAdc;
use esp_idf_hal::adc::{self, Atten11dB};
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::FromValueType;
use esp_idf_hal::{gpio, i2c};
use sx1509;

pub struct Board {
    pub i2c1: i2c::Master<
        i2c::I2C0,
        esp_idf_hal::gpio::Gpio4<gpio::InputOutput>,
        esp_idf_hal::gpio::Gpio5<gpio::Output>,
    >,
    pub adc1: PoweredAdc<adc::ADC1>,
    pub adc1_ch0: gpio::Gpio0<Atten11dB<adc::ADC1>>,
    pub gpio_exp: sx1509::Sx1509<
        i2c::Master<i2c::I2C0, gpio::Gpio4<gpio::InputOutput>, gpio::Gpio5<gpio::Output>>,
    >,
}

impl Board {
    pub fn init(p: Peripherals) -> Board {
        // I2C
        let sda = p.pins.gpio4.into_input_output().unwrap();
        let scl = p.pins.gpio5.into_output().unwrap();
        let i2c = p.i2c0;
        let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());
        let mut i2c1 =
            i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config).unwrap();

        // GPIO expander
        let mut expander = sx1509::Sx1509::new(&mut i2c1, sx1509::DEFAULT_ADDRESS);
        expander.borrow(&mut i2c1).software_reset().unwrap();
        expander.borrow(&mut i2c1).set_bank_a_direction(0).unwrap();
        expander
            .borrow(&mut i2c1)
            .set_bank_b_direction(0xFF)
            .unwrap();

        // ADC
        let adc1_ch0 = p.pins.gpio0.into_analog_atten_11db().unwrap();
        let config = adc::config::Config::new().calibration(true);
        let adc1 = PoweredAdc::new(p.adc1, config).unwrap();

        Board {
            i2c1: i2c1,
            adc1: adc1,
            adc1_ch0: adc1_ch0,
            gpio_exp: expander,
        }
    }
}
