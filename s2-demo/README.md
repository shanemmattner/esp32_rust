# ESP32-S2 Demo

## Pinout
![pinout](./ESP32-S2-DevKit-Lipo-pinout.jpg)

## Github repo for board
[ESP32-S2-DevKit-LiPo](https://github.com/OLIMEX/ESP32-S2-DevKit-LiPo)

## Tasks

- ### Cloud
    - MQTT upload data
    - OTA
- ### Peripherals
    - ADC
    - SPI
    - I2C
    - WIFI
    - GPIO
        - Input
        - Output
    - Interrupt
    - DMA
    - UART
- ### General
    - Unit tests
    - Multi-Threading
    - CLI
    - FreeRTOS

```
export RUST_ESP32_STD_DEMO_WIFI_SSID='yourSSID'
export RUST_ESP32_STD_DEMO_WIFI_PASS='yourPASS'
cargo build
espflash /dev/ttyUSB0 target/xtensa-esp32s2-espidf/debug/s2-demo
espmonitor /dev/ttyACM0
```