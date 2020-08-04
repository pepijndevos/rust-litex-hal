use core::ops::Deref;
use litex_pac::uart::RegisterBlock;

pub struct UART {
    registers: &'static RegisterBlock
}

impl UART {
    pub fn new<UART: Deref<Target=RegisterBlock>>(uart: UART) -> Self {
        Self {
            registers: unsafe { &*(uart.deref() as *const RegisterBlock) }
        }
    }
}

impl embedded_hal::serial::Write<u8> for UART {
    type Error = core::convert::Infallible;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        // Wait until TXFULL is `0`
        if self.registers.txfull.read().bits() != 0 {
            Err(nb::Error::WouldBlock)
        } else {
            unsafe {
                self.registers.rxtx.write(|w| w.rxtx().bits(word.into()));
            }
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.registers.txempty.read().bits() != 0 {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl embedded_hal::blocking::serial::write::Default<u8> for UART {}
