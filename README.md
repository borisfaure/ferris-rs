## Installing

```
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## Compile & Flashing
cargo build --release --target thumbv6m-none-eabi
cargo objcopy --target thumbv6m-none-eabi --release -- -O binary ferris-firmware.bin
dfu-util -d 0483:DF11 -a 0 -s 0x08000000:leave -D ferris-firmware.bin
