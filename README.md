# Serial Scale
An IOT server written in rust for reading weighing data on scales via serial
port.

## Features
 - very fast single binary with no dependencies
 - extensively tested with [hurl](https://github.com/Orange-OpenSource/hurl)
 - currently only supports
[Toledo](https://en.wikipedia.org/wiki/Mettler_Toledo) scales
(pull requests welcome).

## Install
```
cargo install serialscale
```

## Usage
Connects to a scale via USB serial port.

```
sudo serialscale /dev/ttyUSB0
```

You must identify in your operating system, according to the scale's connection
type, which is the correct path.

For more options use:

```
serialscale --help
```

## Development
```
sudo apt install libudev-dev pkg-config
cargo build --release
```

## Releases
Currently, only binaries for generic versions of Linux are distributed across
releases.
```
sudo apt install pkg-config libssl-dev musl-tools
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

## Tests
```
./target/release/serialscale -b 0 tests/test.dat
hurl --test tests/en/basic.hurl

./target/release/serialscale -b 0 tests/test.dat --unit Kg
hurl --test tests/en/basic.hurl

./target/release/serialscale -b 0 tests/test.dat -u Lb --port 8003
hurl --test tests/en/unit_lb.hurl

./target/release/serialscale -b 0 tests/test.dat --min-weight 3.67 -u Kg -p 8004
hurl --test tests/en/min_weight.hurl

./target/release/serialscale -b 0 tests/test.dat --max-weight 0 --port 8005
hurl --test tests/en/max_weight.hurl

./target/release/serialscale -b 0 tests/test.dat --min-tare 6  -p 8006
hurl --test tests/en/min_tare.hurl

./target/release/serialscale -b 0 tests/test.dat --max-tare 5 -u Kg -p 8007
hurl --test tests/en/max_tare.hurl

./target/release/serialscale -b 0 tests/test.dat -l pt -u Kg
hurl --test tests/pt/basic.hurl

./target/release/serialscale -b 0 tests/test.dat -u Lb -l pt --port 8003
hurl --test tests/pt/unit_lb.hurl

./target/release/serialscale -b 0 tests/test.dat --min-weight 3.67 -u Kg -l pt -p 8004
hurl --test tests/pt/min_weight.hurl

./target/release/serialscale -b 0 tests/test.dat --max-weight 0 -l pt --port 8005
hurl --test tests/pt/max_weight.hurl

./target/release/serialscale -b 0 tests/test.dat --min-tare 6 -l pt -p 8006
hurl --test tests/pt/min_tare.hurl

./target/release/serialscale --baud-rate 0 tests/test.dat --max-tare 5 -u Kg --lang pt -p 8007
hurl --test tests/pt/max_tare.hurl
```

## Contributing
It's a very simple project. Any contribution is greatly appreciated.
