# Waterkotte WPCU.C communication tool

TBD

## Build setup

    sudo apt install pkg-config
    sudo apt install libudev-dev

## Cross compilation to Raspberry PI Zero

### Setup

    rustup target add arm-unknown-linux-musleabihf
    git clone https://github.com/raspberrypi/tools $HOME/rpi_tools

### Compilation

    PKG_CONFIG_ALLOW_CROSS=1 \
    RUSTFLAGS="-C linker=$HOME/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc" \
    cargo build --target arm-unknown-linux-musleabihf --tests --release

## Build setup for MacOS X

## Cross compilation to Raspberry PI Zero

Source: https://pixelspark.nl/2020/cross-compiling-rust-programs-for-a-raspberry-pi-from-macos

### Setup

    brew install arm-linux-gnueabihf-binutils
    rustup target add arm-unknown-linux-musleabi

In ~/.cargo/config:

    [target.arm-unknown-linux-musleabi]
    linker = "arm-linux-gnueabihf-ld"
    [target.x86_64-linux-musleabi]
    linker = "x86_64-linux-gnueabihf-ld"

### Compilation

    cargo build --target=arm-unknown-linux-musleabi
