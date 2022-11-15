# SHT2x Sensor Driver

This is a (WIP) sensor driver for the sht2x temperature and humidity sensor

## Usage

```rust
    let mut sht2x = SHT2x::new(i2c, delay);
    let temperature: f32 = sht2x.temperature(); // temperature in degree centigrade
    let rel_hum: f32 = sht2x.humidity();        // relative humidity in %
```

## Roadmap
- [ ] Test if all of the functions work
- [ ] Error handling
- [ ] Add async functions with nohold version of the commands

## License

This project is Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
