#[macro_export]
macro_rules! spi {
    ($(
        $SPIX:ident: ($PACSPIX:ty, $WORD:ty),
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $SPIX {
                registers: $PACSPIX,
            }

            impl $SPIX {
                pub fn new(registers: $PACSPIX) -> Self {
                    Self { registers }
                }

                pub fn free(self) -> $PACSPIX {
                    self.registers
                }
            }

            impl embedded_hal::spi::FullDuplex<$WORD> for $SPIX {
                type Error = core::convert::Infallible;

                fn read(&mut self) -> nb::Result<$WORD, Self::Error> {
                    if self.registers.status.read().done().bit() {
                        Ok(self.registers.miso.read().bits() as $WORD)
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

            impl embedded_hal::blocking::spi::write::Default<u8> for $SPIX {}
            impl embedded_hal::blocking::spi::transfer::Default<u8> for $SPIX {}

            impl From<$PACSPIX> for $SPIX {
                fn from(registers: $PACSPIX) -> $SPIX {
                    $SPIX::new(registers)
                }
            }
        )+
    }
}
