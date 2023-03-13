## Installing the needed tools

Considering one has rust installed by [rustup.rs](https://rustup.rs), then
one has to run the following commands:
```shell
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## Compile & Flashing

```shell
cargo build --release
cargo objcopy --release -- -O binary ferris-firmware.bin
dfu-util -d 0483:DF11 -a 0 -s 0x08000000:leave -D ferris-firmware.bin
```
