# Rust Litex HAL

A Rust embedded HAL crate for [LiteX](https://github.com/enjoy-digital/litex) cores. It contains basic HAL traits for GPIO, UART, SPI, and delay.

![ULX3S demo](http://pepijndevos.nl/images/ulx3s_oled.gif)

More info and instructions [on my blog](http://pepijndevos.nl/2020/08/04/a-rust-hal-for-your-litex-fpga-soc.html) and example project [in this repo](https://github.com/pepijndevos/rust-litex-example/tree/master)

The repository also contains an example that you can run on Verilator using `litex_sim`.

## Compiling and simulating the example

### Compilation

The following dependencies are required to generate Rust code for peripherals (also called Peripheral Access Crate or PAC) and build the example for it.

#### Rust target for RISCV 32I

Our example use VexRiscv, so to be able to compile them you need to add `riscv32i-unknown-none-elf` target for Rust.

```bash
rustup target add riscv32i-unknown-none-elf
```

#### Python

For LiteX scripts.

* ArchLinux:

```bash
sudo pacman -S python
```

* Ubuntu:

```bash
sudo apt install python3
```

* Universal LiteX script

[Official instructions](https://github.com/enjoy-digital/litex#quick-start-guide).

#### LiteX

To build cores and optionally simulate it using [verilator](#verilator).

[Official instructions](https://github.com/enjoy-digital/litex#quick-start-guide).

### Simulation on litex_sim

The following dependencies are required if you want to run the example on `litex_sim`.

#### Cross compiler for RISCV 32I

To compile VexRiscv soft core. RISCV 64 can also build RISCV 32.

* ArchLinux:

```bash
sudo pacman -S riscv64-elf-gcc
```

* Ubuntu:

```bash
sudo apt install gcc-riscv64-unknown-elf
```

#### Verilator

Simulator to run simulation.

* ArchLinux:

```bash
sudo pacman -S verilator
```

* Ubuntu:

```bash
sudo apt install verilator
```

## Simulation

To run the simulation execute the following command:

```bash
cargo xtask simulate --example counter
```

You can also pass `--release` to the simulation command.
