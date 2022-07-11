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

#### LiteX

To build cores and optionally simulate it using [verilator](#verilator).

```bash
# Optionally, you can install this packages into a separate environment
virtualenv litex
source litex/bin/activate

# will download the latest LiteX packages (from Git) and add them to your Python environment:
cd folder_for_litex
wget https://raw.githubusercontent.com/enjoy-digital/litex/master/litex_setup.py
chmod +x litex_setup.py
./litex_setup.py --init --install --user
```

Commands are taken from the [official LiteX quick start guide](https://github.com/enjoy-digital/litex#quick-start-guide).

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

#### LLVM tools

LLVM tools, needed to convert the example from ELF to BIN.

```bash
rustup component add llvm-tools-preview
```

#### cargo-make

Rust task runner and build tool, needed to automate simulation.

* Cargo:

```bash
cargo install --no-default-features cargo-make
```

* ArchLinux (from AUR):

```bash
paru -S cargo-make
```

#### cargo-binutils

Cargo subcommands to invoke the LLVM tools, needed to convert the example from ELF to BIN.

* Cargo:

```bash
cargo install cargo-binutils
```

* ArchLinux (from AUR):

```bash
paru -S cargo-binutils
```

## Simulation

To run the simulation execute the following command:

```bash
cargo make simulate --example counter
```

You can pass additional Cargo flags such as `--release` at the end of the simulation command.
