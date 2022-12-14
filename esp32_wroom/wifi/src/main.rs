// Code copied from https://github.com/snasirca/rust-esp32-c3-blinky/blob/main/src/main.rs
use log::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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

const WIFI_SSID: &str = env!("RUST_ESP32_STD_WIFI_SSID");
const WIFI_PASS: &str = env!("RUST_ESP32_STD_WIFI_PASS");

fn main() {
    esp_idf_sys::link_patches();
    sensible_env_logger::init!();

    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio2.into_output().unwrap();

    let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());

    let _wifi = start_wifi_client(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )
    .unwrap();

    loop {
        println!("Toggle");
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(1000));

        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}

fn start_wifi_client(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    info!("Wifi created");

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: WIFI_SSID.into(),
        password: WIFI_PASS.into(),
        ..Default::default()
    }))?;

    info!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
            _ip_settings,
        ))),
        ApStatus::Stopped,
    ) = status
    {
        info!("Wifi connected");
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}
