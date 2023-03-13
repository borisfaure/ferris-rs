# Rust Firmware for the Ferris keyboard

This firmware written in Rust is targetted for the
[Ferris keyboard](https://github.com/pierrechevalier83/ferris) built with a
STM32F072 MCU and a MCP23017 IO expander.

It is based on the [Keyberon library](https://github.com/TeXitoi/keyberon).

## Features

- Single layer keymap
- And nothing else for the moment

## Installing the needed tools

Considering one has rust installed by [rustup.rs](https://rustup.rs), then
one has to run the following commands:

```shell
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## Compile & Flashing

The name of the model to flash is set a cargo feature. The possible names are:

- `bling`
- `compact`
- `mini`
- `high`

Example for the `mini` model:

```shell
cargo objcopy --release --no-default-features --features="mini" -- -O binary ferris-firmware.bin
dfu-util -d 0483:DF11 -a 0 -s 0x08000000:leave -D ferris-firmware.bin
```
