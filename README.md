# Toledo
An IOT server written in rust for connecting via serial port to toledo scales.

## Dependencies
On ubuntu
```
sudo apt install libudev-dev pkg-config
```

## Releases
Currently, only binaries for generic versions of Linux are distributed across
releases.
```
sudo apt install pkg-config libssl-dev musl-tools
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

## Todo
 - set many attempts to read in serial port 
 - every error message should be unique
 - rename package to serialscale

## Tests
