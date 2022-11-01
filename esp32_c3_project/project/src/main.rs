use embedded_hal::adc::OneShot;
use esp_idf_hal::prelude::*;
use esp_idf_sys as _;
use std::thread;
use std::time::Duration;

mod helpers;
mod init;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // // Initialize peripherals
    let peripherals = Peripherals::take().unwrap();
    let mut board = init::Board::init(peripherals);

    loop {
        let buff = board
            .gpio_exp
            .borrow(&mut board.i2c1)
            .get_bank_a_data()
            .unwrap();
        board
            .gpio_exp
            .borrow(&mut board.i2c1)
            .set_bank_b_data(buff)
            .unwrap();
        log::info!("{:?}", buff);
        log::info!(
            "a1_ch0 sensor reading: {}mV",
            board.adc1.read(&mut board.adc1_ch0).unwrap()
        );
        thread::sleep(Duration::from_millis(100));
    }
}
