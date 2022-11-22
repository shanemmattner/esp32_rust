use embedded_hal_1_0_0::adc::nb::{Channel as Channel1_0_0, OneShot as OneShot1_0_0};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::adc::{self, PoweredAdc};
use esp_idf_hal::gpio::Pin;
use esp_idf_hal::prelude::*;

fn main() {
    esp_idf_sys::link_patches();

    let p = Peripherals::take().unwrap();
    let mut pin = p.pins.gpio34.into_analog_atten_11db().unwrap();
    let config = adc::config::Config::new().calibration(true);
    let mut adc = PoweredAdc::new(p.adc1, config).unwrap();

    esp_idf_sys::esp!(unsafe {
        esp_idf_sys::gpio_set_pull_mode(pin.pin(), esp_idf_sys::gpio_pull_mode_t_GPIO_PULLUP_ONLY)
    })
    .unwrap();

    loop {
        let value = nb::block!(OneShot1_0_0::read(&mut adc, &mut pin)).unwrap();
        println!("val:{}", value);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
