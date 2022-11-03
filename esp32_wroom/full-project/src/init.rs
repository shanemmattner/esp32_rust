use embedded_hal::digital::v2::OutputPin;
use embedded_svc::eth;
use embedded_svc::eth::{Eth, TransitionalState};
use embedded_svc::httpd::registry::*;
use embedded_svc::httpd::*;
use embedded_svc::io;
use embedded_svc::mqtt::client::utils::ConnState;
use embedded_svc::mqtt::client::{Client, Connection, MessageImpl, Publish, QoS};
use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::TimerService;
use embedded_svc::timer::*;
use esp_idf_hal::adc::PoweredAdc;
use esp_idf_hal::adc::{self, Atten11dB};
use esp_idf_hal::delay;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_hal::units::FromValueType;
use esp_idf_hal::{gpio, i2c};
use esp_idf_svc::eventloop::*;
use esp_idf_svc::httpd as idf;
use esp_idf_svc::httpd::ServerRegistry;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::*;
use esp_idf_sys::EspError;
use sx1509;

const MQTT_HOST: &str = env!("RUST_ESP32_MQTT_HOST");
const MQTT_PORT: &str = env!("RUST_ESP32_MQTT_PORT");

pub struct Board {
    // pub i2c1: i2c::Master<i2c::I2C0, gpio::Gpio4<gpio::InputOutput>, gpio::Gpio5<gpio::Output>>,
    // pub adc1: PoweredAdc<adc::ADC1>,
    // pub adc1_ch0: gpio::Gpio0<Atten11dB<adc::ADC1>>,
    // pub gpio_exp: sx1509::Sx1509<
    //     i2c::Master<i2c::I2C0, gpio::Gpio4<gpio::InputOutput>, gpio::Gpio5<gpio::Output>>,
    // >,
    // pub psh_btn: gpio::Gpio1<gpio::Input>,
    pub led: gpio::Gpio2<gpio::Output>,
}

impl Board {
    pub fn init(p: Peripherals) -> Board {
        // GPIO
        // let btn = p.pins.gpio1.into_input().unwrap();
        let led = p.pins.gpio2.into_output().unwrap();

        // // I2C
        // let sda = p.pins.gpio4.into_input_output().unwrap();
        // let scl = p.pins.gpio5.into_output().unwrap();
        // let i2c = p.i2c0;
        // let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());
        // let mut i2c1 =
        //     i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config).unwrap();

        // // GPIO expander
        // let mut expander = sx1509::Sx1509::new(&mut i2c1, sx1509::DEFAULT_ADDRESS);
        // expander.borrow(&mut i2c1).software_reset().unwrap();
        // expander.borrow(&mut i2c1).set_bank_a_direction(0).unwrap();
        // expander
        //     .borrow(&mut i2c1)
        //     .set_bank_b_direction(0xFF)
        //     .unwrap();

        // // ADC
        // let adc1_ch0 = p.pins.gpio0.into_analog_atten_11db().unwrap();
        // let config = adc::config::Config::new().calibration(true);
        // let adc1 = PoweredAdc::new(p.adc1, config).unwrap();

        // SPI

        // let sclk =
        // let sdo =
        // let sdi =
        // let cs =
        // let config = <spi::config::Config as Default>::default().baudrate(26.MHz().into());
        // let di = SPIInterfaceNoCS::new(
        //     spi::Master::<spi::SPI2, _, _, _, _>::new(
        //         spi,
        //         spi::Pins {
        //             sclk,
        //             sdo,
        //             sdi: Option::<gpio::Gpio21<gpio::Unknown>>::None,
        //             cs: Some(cs),
        //         },
        //         config,
        //     )?,
        //     dc.into_output()?,
        // );

        Board {
            // i2c1: i2c1,
            // adc1: adc1,
            // adc1_ch0: adc1_ch0,
            // gpio_exp: expander,
            // psh_btn: btn,
            led: led,
        }
    }
}
