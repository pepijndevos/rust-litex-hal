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
                    registers.ev_enable().write(|w| {
                        w.rx().set_bit()
                    });
                    Self { registers }
                }

                pub fn free(self) -> $PACUARTX {
                    self.registers
                }
            }

            impl $crate::hal::serial::Write<u8> for $UARTX {
                type Error = core::convert::Infallible;

                fn write(&mut self, word: u8) -> $crate::nb::Result<(), Self::Error> {
                    // Wait until TXFULL is `0`
                    if self.registers.txfull().read().bits() != 0 {
                        Err($crate::nb::Error::WouldBlock)
                    } else {
                        unsafe {
                            self.registers.rxtx().write(|w| w.rxtx().bits(word.into()));
                        }
                        Ok(())
                    }
                }
                fn flush(&mut self) -> $crate::nb::Result<(), Self::Error> {
                    if self.registers.txempty().read().bits() != 0 {
                        Ok(())
                    } else {
                        Err($crate::nb::Error::WouldBlock)
                    }
                }
            }
            
            impl $crate::hal::serial::Read<u8> for $UARTX {
                type Error = core::convert::Infallible;
                
                fn read(&mut self) -> $crate::nb::Result<u8, Self::Error> {
                    // Wait until RXEMPTY is `0`
                    if self.registers.rxempty().read().bits() != 0 {
                        Err($crate::nb::Error::WouldBlock)
                    } else {
                        let result = unsafe {
                            Ok(self.registers.rxtx().read().bits() as u8)
                        }
                        self.registers.ev_pending().write(|w| w.rx().set_bit());
                        return result;
                    }
                }
            }

            impl $crate::hal::blocking::serial::write::Default<u8> for $UARTX {}

            impl core::fmt::Write for $UARTX {
                fn write_str(&mut self, s: &str) -> core::fmt::Result {
                    use $crate::hal::prelude::*;
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
