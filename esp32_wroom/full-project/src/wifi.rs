use anyhow::bail;
use anyhow::Result;
use embedded_svc::ipv4;
use embedded_svc::ping::Ping;
use embedded_svc::wifi::*;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::ping;
use esp_idf_svc::sntp;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::wifi::*;
use std::{cell::RefCell, env, sync::atomic::*, sync::Arc, thread, time::*};

const WIFI_SSID: &str = env!("RUST_ESP32_STD_WIFI_SSID");
const WIFI_PASS: &str = env!("RUST_ESP32_STD_WIFI_PASS");

pub fn wifi_init() {
    let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());

    let _wifi = start_wifi_client(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    )
    .unwrap();
}

fn start_wifi_client(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    println!("Wifi created");

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: WIFI_SSID.into(),
        password: WIFI_PASS.into(),
        ..Default::default()
    }))?;

    println!("Wifi configuration set, about to get status");

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
        println!("Wifi connected");
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
