#[macro_export]
macro_rules! uart {
    ($(
        $UARTX:ident: $PACUARTX:ty,
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $UARTX {
                registers: $PACUARTX,
            }

            impl $UARTX {
                pub fn new(registers: $PACUARTX) -> Self {
                    Self { registers }
                }

                pub fn free(self) -> $PACUARTX {
                    self.registers
                }
            }

            impl embedded_hal::serial::Write<u8> for $UARTX {
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

            impl embedded_hal::blocking::serial::write::Default<u8> for $UARTX {}

            impl core::fmt::Write for $UARTX {
                fn write_str(&mut self, s: &str) -> core::fmt::Result {
                    use embedded_hal::prelude::*;
                    self.bwrite_all(s.as_bytes()).ok();
                    Ok(())
                }
            }

            impl From<$PACUARTX> for $UARTX {
                fn from(registers: $PACUARTX) -> $UARTX {
                    $UARTX::new(registers)
                }
            }
        )+
    }
}
