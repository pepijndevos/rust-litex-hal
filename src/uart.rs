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

            #[derive(Debug)]
            pub enum UartError {
                InvalidState
            }

            impl $crate::hal_io::Error for UartError {
                fn kind(&self) -> $crate::hal_io::ErrorKind {
                    $crate::hal_io::ErrorKind::Other
                }
            }

            impl $crate::hal_io::ErrorType for $UARTX {
                type Error = UartError;
            }

            impl $UARTX {
                pub fn new(registers: $PACUARTX) -> Self {
                    Self { registers }
                }

                pub fn free(self) -> $PACUARTX {
                    self.registers
                }

                fn tx_ready(&self) -> bool {
                    self.registers.txfull().read().bits() == 0
                }

                fn write_char(&mut self, word: &u8) -> () {

                    // Wait until TXFULL is `0`
                    while !self.tx_ready() {}
                    unsafe {
                        self.registers.rxtx().write(|w| w.rxtx().bits(*word));
                    }
                }
            }


            impl $crate::hal_io::Write for $UARTX {

                fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
                    for word in buf.iter() {
                        self.write_char(word);
                    }
                    Ok(buf.len())
                }
                fn flush(&mut self) -> Result<(), Self::Error> {
                    while !self.tx_ready() {}
                    Ok(())
                }
            }

            impl $crate::hal_io::Read for $UARTX {

                fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
                    let mut i = 0;
                    for word in buf.iter_mut() {
                        while self.registers.rxempty().read().bits() != 0 {}

                        *word = unsafe {
                            self.registers.rxtx().read().bits() as u8
                        };
                        self.registers.ev_pending().write(|w| w.rx().set_bit());
                        i += 1;
                    }
                    Ok(i)
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
