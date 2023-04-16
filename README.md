![CI](https://github.com/borisfaure/ferris-rs/actions/workflows/ci.yml/badge.svg)

# Rust Firmware for the Ferris keyboard

This firmware written in Rust is targetted for the
[Ferris keyboard](https://github.com/pierrechevalier83/ferris) built with a
STM32F072 MCU and a MCP23017 IO expander.

It is based on the [Keyberon library](https://github.com/TeXitoi/keyberon).

## Features

- Multi layers keymaps
- Multiple keymaps
- Different Ferris models
- Hold Tap actions

## What's missing

- No support for controlling the mouse
- No RGB (support is in keyberon but not implemented here)
- Sequences
- One Shot Actions
- ...

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

The possible keymaps are:

- `keymap_basic`
- `keymap_borisfaure`
- `keymap_pierrec83`

Some of them were converted with help from the tool
[qmk-layout-to-keyberon](https://github.com/borisfaure/qmk-layout-to-keyberon).


In order to generate and install the firmware for the `mini` model and the
keymap `keymap_basic`:

```shell
cargo objcopy --release --no-default-features --features="mini,keymap_basic" -- -O binary ferris-firmware.bin
dfu-util -d 0483:DF11 -a 0 -s 0x08000000:leave -D ferris-firmware.bin
```
