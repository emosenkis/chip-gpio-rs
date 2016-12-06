# chip-gpio
Access NTC CHIP GPIO pins by name.

[![docs](https://docs.rs/chip-gpio/badge.svg)](https://docs.rs/chip-gpio)
[![crates.io](https://img.shields.io/crates/v/chip-gpio.svg)](https://crates.io/crates/chip-gpio)

Usage:

```rust
extern crate chip_gpio;

use chip_gpio::ChipPin::*;

let pin = XIO_P0.get();
```
