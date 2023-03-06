# Formation Herkulex
This repository contains programs shown during the training I gave at [Club Robot](https://clubrobotinsat.github.io) at INSA Toulouse.
The aim of the training is to introduce new members to the Rust language, 
to electrics for robotics and to the Herkulex servomotors library that I have developed.

## Dependencies
- Linux based OS
- git
- rust | cargo (https://www.rust-lang.org/)
- IDE with Rust support
- OpenOCD
- gdb-multiarch (Debian-based OS, find equivalent for others)
- cargo generate

## Building and flashing
Go to the project path using a terminal ([see this cheatsheet if necessary](https://www.guru99.com/linux-commands-cheat-sheet.html))

Then build the project using the following command: `cargo build`.

Then open the debugger in another terminal: `openocd -f black_pill.cfg`.

In the first terminal, use the following command to flash and run `./flash-and-run.sh <<target file>>`.

The target file path should be something like : `target/thumbv7m-none-eabi/debug/herkulex-formation-td`

## Wiring
See our documentation (Notion)
Main important information:
- 7.4V for Herkulex
- 5V for STM32
- Pull-up resistor for the RX channel
- Verify that mass is wired everywhere it should be
- Check that your DC generator is turned on...

# cortex-m-quickstart

This project uses the cortex-m-quickstart repository.