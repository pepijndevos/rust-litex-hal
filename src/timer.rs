#[macro_export]
macro_rules! timer {
    ($(
        $TIMERX:ident: $PACTIMERX:ty,
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $TIMERX {
                registers: $PACTIMERX,
                pub sys_clk: u32,
            }

            impl $TIMERX {
                pub fn new(registers: $PACTIMERX, sys_clk: u32) -> Self {
                    Self { registers, sys_clk }
                }

                pub fn free(self) -> $PACTIMERX {
                    self.registers
                }
            }

            impl $crate::hal::delay::DelayNs for $TIMERX {
                fn delay_ns(&mut self, ns: u32) -> () {
                    let nanos_per_clk: u32 = 1_000_000_000 / self.sys_clk;
                    // Round up to nearest clock cycle increment.
                    let value: u32 =  (ns / nanos_per_clk) + 1;
                    unsafe {
                        self.registers.en().write(|w| w.bits(0));
                        self.registers.reload().write(|w| w.bits(0));
                        self.registers.load().write(|w| w.bits(value));
                        self.registers.en().write(|w| w.bits(1));
                        self.registers.update_value().write(|w| w.bits(1));
                        while self.registers.value().read().bits() > 0 {
                            self.registers.update_value().write(|w| w.bits(1));
                        }
                    }
                }
            }
        )+
    }
}
