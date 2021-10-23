# STM32 bootie

The _very_ simplest bootloader example. Boots directly into bootloader, then loads a blinky example.

## Building

You'll need the rust embedded toolchain: https://docs.rust-embedded.org/book/intro/install.html

The point here is that the booloader sits in the start of the flash memory address space:

```
// bootloader/memory.x
FLASH : ORIGIN = 0x08000000, LENGTH = 32K
```

The blblinky example code starts two pages later:

```
// blblinky/memory.x
FLASH : ORIGIN = 0x08008000, LENGTH = 96K
```

In total, 128K of flash memory can be used (32K for bootloader, 96K for user application), your MCU might have more memory.

The bootloader code (`main.rs`), does nothing more than executing the code starting at memory address `0x08008000`, but you can now adjust it to your liking.

You need to flash both projects successively to run the code, the order doesn't really matter. Use `cargo flash` if possible and make sure the whole chip is not erased when uploading the binaries (should not be set be default).

## Next steps

It would be good to showcase a method to write binary code to the flash memory to do OTA updates. For an extensive example see here: https://github.com/karthickai/rustboot


## LICENSE

See the bootloader and blblink folders
