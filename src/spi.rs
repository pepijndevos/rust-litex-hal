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

            #[derive(Debug, Copy, Clone, Eq, PartialEq)]
            pub enum SpiError {
                TransactionFailed,
                InvalidState,
            }
            impl $crate::hal::spi::Error for SpiError {
                fn kind(&self) -> $crate::hal::spi::ErrorKind {
                    match *self {
                        SpiError::TransactionFailed => $crate::hal::spi::ErrorKind::Other,
                        SpiError::InvalidState => $crate::hal::spi::ErrorKind::Other
                    }
                }
            }

            impl $crate::hal::spi::ErrorType for $SPIX {
                type Error = SpiError;
            }

            impl $SPIX {
                pub fn new(registers: $PACSPIX) -> Self {
                    Self { registers }
                }

                pub fn free(self) -> $PACSPIX {
                    self.registers
                }

                fn write_one(&mut self, word: &$WORD) -> Result<(), SpiError> {
                    if self.registers.status().read().done().bit() {
                        unsafe {
                            self.registers.mosi().write(|w| w.bits(*word as u32));
                            self.registers.control().write(|w| {
                                w.length().bits(core::mem::size_of::<$WORD>()).start().bit(true)
                            });
                        }
                        Ok(())
                    } else {
                        Err(SpiError::InvalidState)
                    }
                }

                fn is_done(&mut self) -> bool {
                    self.registers.status().read().done().bit()
                }

                fn read_priv(&mut self, bufs: &mut [$WORD]) ->  Result<(), SpiError> {
                    if self.registers.status().read().done().bit() {
                        for buf in bufs.iter_mut() {
                            unsafe {
                                self.registers.control().write(|w| {
                                    w.length().bits(core::mem::size_of::<$WORD>()).start().bit(true)
                                });
                            }
                            while !self.is_done() {}
                            *buf = self.registers.miso().read().bits() as $WORD;
                        }
                        Ok(())
                    } else {
                        Err(SpiError::InvalidState)
                    }
                }

                fn write_priv(&mut self, words: &[$WORD]) -> Result<(), SpiError> {
                    if self.registers.status().read().done().bit() {
                        for word in words.iter() {
                            self.write_one(word)?;
                            while !self.is_done() {}
                        }
                        Ok(())
                    } else {
                        Err(SpiError::InvalidState)
                    }
                }

                fn transfer_priv(&mut self, read: &mut [$WORD], write: &[$WORD]) -> Result<(), SpiError> {
                    let len = read.len().max(write.len());
                    for i in 0..len {
                        let wb = write.get(i).copied().unwrap_or(0);

                        self.write_one(&wb)?;
                        while !self.is_done() {}
                        let rb = self.registers.miso().read().bits() as $WORD;
                        if let Some(r) = read.get_mut(i) {
                            *r = rb;
                        }
                    }
                    Ok(())
                }

                fn transfer_in_place_priv(&mut self, words: &mut [$WORD]) -> Result<(), SpiError> {
                    if self.is_done() {
                        for word in words.iter_mut() {
                            self.write_one(word)?;
                            while !self.is_done() {}
                            *word = self.registers.miso().read().bits() as $WORD;
                        }
                        Ok(())
                    } else {
                        Err(SpiError::InvalidState)
                    }
                }
            }

            impl $crate::hal::spi::SpiDevice<$WORD> for $SPIX {

                fn transaction(&mut self, operations: &mut [$crate::hal::spi::Operation<'_, $WORD>]) -> Result<(), Self::Error> {
                    for op in operations {
                        match op {
                            $crate::hal::spi::Operation::Read(buf) => self.read_priv(buf)?,
                            $crate::hal::spi::Operation::Write(buf) => self.write_priv(buf)?,
                            $crate::hal::spi::Operation::Transfer(read_buf, write_buf) => self.transfer_priv(read_buf, write_buf)?,
                            $crate::hal::spi::Operation::TransferInPlace(buf) => self.transfer_in_place(buf)?,
                            $crate::hal::spi::Operation::DelayNs(_) => continue,
                        }
                    }
                    Ok(())
                }
            }
        )+
    };
}
