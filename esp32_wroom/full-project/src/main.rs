#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use anyhow::bail;
use anyhow::Result;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_svc::wifi::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::wifi::*;
use log::*;
use mqttrust::encoding::v4::Pid;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
extern crate freertos_rs;
use freertos_rs::*;

mod init;
mod mqtt;
mod timers;
mod wifi;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    sensible_env_logger::init!();

    // // Initialize peripherals
    let peripherals = Peripherals::take().unwrap();

    let mut board = init::Board::init(peripherals);
    let _wifi = wifi::wifi_init();

    mqtt::mqtt_init();

    let task1 = Task::new()
        .name("hello")
        .stack_size(128)
        .start(|| loop {
            println!("Hello world!");
            CurrentTask::delay(freertos_rs::Duration::ms(1000));
        })
        .unwrap();

    loop {
        thread::sleep(Duration::from_millis(1000));

        // let buff = board
        //     .gpio_exp
        //     .borrow(&mut board.i2c1)
        //     .get_bank_a_data()
        //     .unwrap();

        // board
        //     .gpio_exp
        //     .borrow(&mut board.i2c1)
        //     .set_bank_b_data(buff)
        //     .unwrap();

        // if board.psh_btn.is_low().unwrap() {
        //     log::println!(
        //         "a1_ch0 sensor reading: {}mV",
        //         board.adc1.read(&mut board.adc1_ch0).unwrap()
        //     );
        board.led.set_high().unwrap();
        // } else {
        //     board.led.set_low().unwrap();
        // }

        thread::sleep(Duration::from_millis(1000));
        board.led.set_low().unwrap();
    }
}
