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

    // // Initialize
    let per = Peripherals::take().unwrap();

    let mut i2c = init::i2c_peripheral(per);

    let mut expander = init::sx1509_init(&mut i2c);

    loop {
        let buff = expander.borrow(&mut i2c).get_bank_a_data().unwrap();
        expander.borrow(&mut i2c).set_bank_b_data(buff).unwrap();
        log::info!("{:?}", buff);
        thread::sleep(Duration::from_millis(100));
    }
}
