# ESP32

## Tasks

- ### Cloud
    - MQTT
        - ~~Connect~~
        - Upload data
        - Receive data
    - OTA
- ### Peripherals
    - ADC
    - Timers
    - SPI
    - I2C
    - ~~WIFI~~
    - GPIO
        - Input
        - ~~Output~~
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
    - Pub/Sub

```
export RUST_ESP32_STD_DEMO_WIFI_SSID='yourSSID'
export RUST_ESP32_STD_DEMO_WIFI_PASS='yourPASS'
cargo build
espflash /dev/ttyUSB0 target/xtensa-esp32-espidf/debug/full-project
espmonitor /dev/ttyUSB0
```