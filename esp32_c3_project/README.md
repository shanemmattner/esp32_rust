# Rust on ESP32
This repo is for me to explore Rust on the ESP32

## Steps to set up a new ESP32 Rust project
1. [Install](https://github.com/esp-rs/rust-build) Rust and Xtensa build tools
    - Make sure to `sudo chmod +x export-esp.sh`
2. Start a project using the [esp-idf-template](https://github.com/esp-rs/esp-idf-template) from the private repo home `dir`. I chose all the default options
```
cargo generate --vcs none --git https://github.com/esp-rs/esp-idf-template cargo
```
3. Build the `Hello World` program by running `cargo build` in the new project dir. This will take a while to build the first time:
```
cd esp32-rust
cargo build
...
Finished dev [optimized + debuginfo] target(s) in 6m 40s
```
4. Flash the ESP32 with the build artifact:
```
espflash /dev/ttyACM0 target/riscv32imc-esp-espidf/debug/project
```
5. Connect to ESP32 and monitor
```
espmonitor /dev/ttyACM0
```

## Links
- [Embedded rust: compiling for ESP32 devices and creating a simple blinky](https://www.youtube.com/watch?v=Sm413MNQE_A)
- [snasirca/rust-esp32-c3-blinky](https://github.com/snasirca/rust-esp32-c3-blinky)
- [ivmarkov/rust-esp32-ulp-blink](https://github.com/ivmarkov/rust-esp32-ulp-blink)
- [Rust - building a UI to plot a sensor value in real time](https://www.youtube.com/watch?v=zUvHkkkrmIY)






## Tasks

- ### Cloud
    - MQTT upload data
    - OTA
- ### Peripherals
    - ADC
    - SPI
    - ~~I2C~~
    - WIFI
    - GPIO
    - Interrupt
    - DMA
- ### General
    - Unit tests
    - Multi-Threading
