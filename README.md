# ESP32 Rust

This is my repo for exploring Rust on ESP32 boards.  The 3 folders below contain:

- `esp32_c3_project`: Folder that contains the package `project` that is the latest work-in-progress using the [Olimex ESP32-C3-DevKit-Lipo](https://www.olimex.com/Products/IoT/ESP32-C3/ESP32-C3-DevKit-Lipo/open-source-hardware)
- `esp32_wroom`: Folder containing different packages for each peripheral.  Most of them work



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
- [ESP32-C3-DevKit-Lipo](https://www.olimex.com/Products/IoT/ESP32-C3/ESP32-C3-DevKit-Lipo/open-source-hardware)
- [Discovery](https://github.com/rust-embedded/discovery)
- [rust-esp32-std-demo](https://github.com/ivmarkov/rust-esp32-std-demo)
- [Ferrous Systems ESP32 Training](https://www.espressif.com/en/news/ESP_RUST_training)
- [awesome-esp-rust](https://github.com/esp-rs/awesome-esp-rust)
- [The Rust on ESP Book](https://esp-rs.github.io/book/introduction.html)
- [Andrei Litvin Youtube channel](https://www.youtube.com/c/AndreiLitvinCa)
- [esp32-rust-nostd-temperature-logger](https://github.com/bjoernQ/esp32-rust-nostd-temperature-logger)
- [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust)
- [Discovery microbit](https://docs.rust-embedded.org/discovery/microbit/)
- [Embedded Rust on Espressif](https://espressif-trainings.ferrous-systems.com/)
- [embedded-trainings-2020](https://github.com/ferrous-systems/embedded-trainings-2020)
- [dsp-discoveryf4-rust](https://github.com/jacobrosenthal/dsp-discoveryf4-rust/)
- [Vers Binarii Embedded Rust](https://www.youtube.com/playlist?list=PLP_X41VhYn5X6Wwjnm0bRwI3n2pdaszxU)
- [The Embedonomicon](https://docs.rust-embedded.org/embedonomicon/index.html)
- [Embedded devices Working Group](https://github.com/rust-embedded/wg)
- [esp32-adc](https://github.com/tecywiz121/esp32-adc)
- [espup](https://github.com/esp-rs/espup)
- [element chat group](https://app.element.io/#/room/#esp-rs:matrix.org)
- [Async Rust vs RTOS showdown!](https://tweedegolf.nl/en/blog/65/async-rust-vs-rtos-showdown)
- [HashMismatch website](http://www.hashmismatch.net/libraries/)