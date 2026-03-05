# DIY vape external module

Seeed Xiao nRF52840 Plus + SSD1306 + TP4056 + IRLB3034 + BC557 + BC547 + TTP223

### Setup
```
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils
cargo install uf2conv
```

### Build
```
cargo build --release
cargo objcopy --release -- -O binary firmware.bin
uf2conv firmware.bin --family 0xADA52840 --base 0x27000 --output firmware.uf2
```

### Info
```
arm-none-eabi-size target/thumbv7em-none-eabihf/release/vape
arm-none-eabi-objdump -h target/thumbv7em-none-eabihf/release/vape

```

### Flash (MacOS)
```
cp firmware.uf2 /Volumes/XIAO-BOOT
```
it's ok:
```
cp: /Volumes/XIAO-BOOT/firmware.uf2: fcopyfile failed: Input/output error
cp: /Volumes/XIAO-BOOT/firmware.uf2: fchmod failed: No such file or directory
```

## Erase
```
probe-rs erase --chip nRF52840_xxAA
```

## Recover
https://github.com/adafruit/Adafruit_nRF52_Bootloader
```
probe-rs download --binary-format hex --chip nRF52840_xxAA bootloader.hex
```
