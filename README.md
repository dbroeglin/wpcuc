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
