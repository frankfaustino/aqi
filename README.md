just playing around with rust

## requirements

- [Rust](https://www.rust-lang.org/)
- [AQI Open Data Platform token](https://aqicn.org/data-platform/token/)

## run

```bash
git clone https://github.com/frankfaustino/aqi.git

cd aqi

cargo build

# get AQI for specified city
cargo run -- --token TOKEN info "Las Vegas"

# get cities for specified region
cargo run -- --token TOKEN search "Philippines"
```
