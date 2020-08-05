# Rust Litex HAL
A Rust embedded HAL crate for LiteX cores.
Allows you to reuse embedded Rust crates on your custom SoC.

![ULX3S demo](http://pepijndevos.nl/images/ulx3s_oled.gif)

Info and instructions [on my blog](http://pepijndevos.nl/2020/08/04/a-rust-hal-for-your-litex-fpga-soc.html), example usage [in this repo](https://github.com/pepijndevos/rust-litex-example/tree/master)

The crate name is very, eh, aspirational. It contains basic HAL traits for GPIO, UART, SPI, and delay.
It is the result of curiosity-driven development, so any future updates are at the mercy of my curiosity, pull requests, or someone hiring me to build them a system that needs a LiteX SoC ;-)
