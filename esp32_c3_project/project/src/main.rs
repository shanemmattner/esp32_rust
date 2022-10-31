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

    // let sda = per.pins.gpio4.into_input_output().unwrap();
    // let scl = per.pins.gpio5.into_output().unwrap();
    // let i2c = per.i2c0;

    // let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());
    // let mut i2c =
    //     i2c::Master::<i2c::I2C0, _, _>::new(i2c, i2c::MasterPins { sda, scl }, config).unwrap();
    let mut i2c = init::i2c_peripheral(per);

    helpers::ptype(&i2c);
    let mut expander = sx1509::Sx1509::new(&mut i2c, sx1509::DEFAULT_ADDRESS);
    // ptype(&expander);
    expander.borrow(&mut i2c).software_reset().unwrap();
    expander.borrow(&mut i2c).set_bank_a_direction(0).unwrap();
    expander
        .borrow(&mut i2c)
        .set_bank_b_direction(0xFF)
        .unwrap();

    loop {
        let buff = expander.borrow(&mut i2c).get_bank_a_data().unwrap();
        // helpers::ptype(&buff);
        expander.borrow(&mut i2c).set_bank_b_data(buff).unwrap();
        log::info!("{:?}", buff);
        thread::sleep(Duration::from_millis(100));
    }
}
