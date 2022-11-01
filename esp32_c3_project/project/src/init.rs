use anyhow::bail;
use embedded_svc::eth;
use embedded_svc::eth::{Eth, TransitionalState};
use embedded_svc::httpd::registry::*;
use embedded_svc::httpd::*;
use embedded_svc::io;
use embedded_svc::ipv4;
use embedded_svc::mqtt::client::{Client, Connection, MessageImpl, Publish, QoS};
use embedded_svc::ping::Ping;
use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::TimerService;
use embedded_svc::timer::*;
use embedded_svc::wifi::*;
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
use esp_idf_svc::mqtt::client::*;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::ping;
use esp_idf_svc::sntp;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::*;
use esp_idf_svc::wifi::*;
use std::{cell::RefCell, env, sync::atomic::*, sync::Arc, thread, time::*};
use sx1509;

const SSID: &str = env!("RUST_ESP32_STD_DEMO_WIFI_SSID");
const PASS: &str = env!("RUST_ESP32_STD_DEMO_WIFI_PASS");

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

        // WiFi
        let netif_stack = Arc::new(EspNetifStack::new().unwrap());
        let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
        let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());
        let mut wifi = wifi(
            netif_stack.clone(),
            sys_loop_stack.clone(),
            default_nvs.clone(),
        )
        .unwrap();

        #[cfg(not(feature = "qemu"))]
        #[cfg(esp_idf_lwip_ipv4_napt)]
        enable_napt(&mut wifi).unwrap();

        Board {
            i2c1: i2c1,
            adc1: adc1,
            adc1_ch0: adc1_ch0,
            gpio_exp: expander,
        }
    }
}

#[cfg(not(feature = "qemu"))]
#[allow(dead_code)]
fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs).unwrap());

    println!("Wifi created, about to scan");

    let ap_printlns = wifi.scan().unwrap();

    let ours = ap_printlns.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        println!(
            "Found configured access point {} on channel {}",
            SSID, ours.channel
        );
        Some(ours.channel)
    } else {
        println!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            SSID
        );
        None
    };

    wifi.set_configuration(&Configuration::Mixed(
        ClientConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            channel,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    ))
    .unwrap();

    println!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))
        .unwrap();

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(ip_settings))),
        ApStatus::Started(ApIpStatus::Done),
    ) = status
    {
        println!("Wifi connected");

        ping(&ip_settings).unwrap();
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

#[cfg(not(feature = "qemu"))]
#[cfg(esp_idf_lwip_ipv4_napt)]
fn enable_napt(wifi: &mut EspWifi) -> Result<()> {
    wifi.with_router_netif_mut(|netif| netif.unwrap().enable_napt(true));

    println!("NAPT enabled on the WiFi SoftAP!");

    Ok(())
}

fn ping(ip_settings: &ipv4::ClientSettings) -> Result<()> {
    println!("About to do some pings for {:?}", ip_settings);

    let ping_summary =
        ping::EspPing::default().ping(ip_settings.subnet.gateway, &Default::default())?;
    if ping_summary.transmitted != ping_summary.received {
        bail!(
            "Pinging gateway {} resulted in timeouts",
            ip_settings.subnet.gateway
        );
    }

    println!("Pinging done");

    Ok(())
}
