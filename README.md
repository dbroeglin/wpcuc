# Waterkotte WPCU.C communication tool

TBD


## Cross compilation to Raspberry PI Zero

### Setup

Did not work (produces an ARMv7 binary): 
    rustup target add arm-unknown-linux-gnueabihf
    sudo apt install g++-arm-linux-gnueabihf

### Compilation

    rustup target add arm-unknown-linux-gnueabi
    git clone https://github.com/raspberrypi/tools $HOME/rpi_tools
    RUSTFLAGS="-C linker=$HOME/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc" cargo build --target arm-unknown-linux-gnueabihf --tests --release
