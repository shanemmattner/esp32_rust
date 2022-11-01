# ESP32-C3

## Tasks

- ### Cloud
    - MQTT upload data
    - OTA
- ### Peripherals
    - ~~ADC~~
    - SPI
    - ~~I2C~~
    - WIFI
    - GPIO
    - Interrupt
    - DMA
- ### General
    - Unit tests
    - Multi-Threading
    - CLI
    - FreeRTOS

```
export RUST_ESP32_STD_DEMO_WIFI_SSID='yourSSID'
export RUST_ESP32_STD_DEMO_WIFI_PASS='yourPASS'
cargo build
espflash /dev/ttyACM0 target/riscv32imc-esp-espidf/debug/project
espmonitor /dev/ttyACM0
```