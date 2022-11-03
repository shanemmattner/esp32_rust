# ESP32

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
    - Error handling
        - Replace `Unwrap()`

```
export RUST_ESP32_STD_DEMO_WIFI_SSID='yourSSID'
export RUST_ESP32_STD_DEMO_WIFI_PASS='yourPASS'
cargo build
espflash /dev/ttyUSB0 target/xtensa-esp32-espidf/debug/full-project
espmonitor /dev/ttyUSB0
```