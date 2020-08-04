use core::ops::Deref;
use litex_pac::oled_spi::RegisterBlock;

pub struct SPI {
    registers: &'static RegisterBlock
}

impl SPI {
    pub fn new<SPI: Deref<Target=RegisterBlock>>(spi: SPI) -> Self {
        Self {
            registers: unsafe { &*(spi.deref() as *const RegisterBlock) }
        }
    }
}

impl embedded_hal::spi::FullDuplex<u8> for SPI {
    type Error = core::convert::Infallible;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if self.registers.status.read().done().bit() {
            Ok(self.registers.miso.read().bits() as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.registers.status.read().done().bit() {
            unsafe {
                self.registers.mosi.write(|w| w.bits(word.into()));
                self.registers.control.write(|w| {
                    w.length().bits(8).start().bit(true)
                });
            }
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl embedded_hal::blocking::spi::write::Default<u8> for SPI {}
//impl embedded_hal::blocking::spi::write_iter::Default<u8> for SPI {}
impl embedded_hal::blocking::spi::transfer::Default<u8> for SPI {}
