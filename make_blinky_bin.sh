#!/bin/sh

# execute from root dir
D=$(pwd)

cd ./blblinky/target/thumbv7em-none-eabihf/release
arm-none-eabi-objcopy -O binary blblinky blblinky.bin
cd $D/file_reader/target/release
./file_reader > $D/bootloader/src/blinky_arr.rs
