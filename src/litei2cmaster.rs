/*
HAL I2C implementation of LiteX litei2c I2C master IP.
Based on the Zephyr RTOS driver by Vogl Electronic GmbH written in C (Apache 2.0 license)
*/

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LiteI2CSpeedMode {
    Standard,
    Fast,
    FastPlus,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LiteI2CError {
    NACK,
    Other,
    // ...
}

impl core::fmt::Display for LiteI2CSpeedMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            LiteI2CSpeedMode::Standard => {
                write!(f, "LiteI2CSpeedMode::Standard")
            }
            LiteI2CSpeedMode::Fast => {
                write!(f, "LiteI2CSpeedMode::Fast")
            }
            LiteI2CSpeedMode::FastPlus => write!(f, "LiteI2CSpeedMode::FastPlus"),
        }
    }
}

impl core::error::Error for LiteI2CError {}

impl core::fmt::Display for LiteI2CError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            LiteI2CError::NACK => {
                write!(f, "LiteI2CError: NACK receied!")
            }
            LiteI2CError::Other => write!(f, "LiteI2CError: Unknown error!"),
        }
    }
}

impl embedded_hal::i2c::Error for LiteI2CError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        match *self {
            LiteI2CError::NACK => embedded_hal::i2c::ErrorKind::NoAcknowledge(
                embedded_hal::i2c::NoAcknowledgeSource::Unknown,
            ),
            _ => embedded_hal::i2c::ErrorKind::Other,
        }
    }
}

#[macro_export]
macro_rules! litei2cmaster {
    ($(
        $LITEI2CMASTER:ident: $PACI2CMASTERX:ty,
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $LITEI2CMASTER {
                registers: $PACI2CMASTERX,
                pub speed_mode: $crate::litei2cmaster::LiteI2CSpeedMode,
            }

            impl $LITEI2CMASTER {
                pub fn new(registers: $PACI2CMASTERX, speed_mode: $crate::litei2cmaster::LiteI2CSpeedMode) -> Self {

                    let speed_mode_reg_setting : u32 =
                        match speed_mode {
                            $crate::litei2cmaster::LiteI2CSpeedMode::Standard => {0},
                            $crate::litei2cmaster::LiteI2CSpeedMode::Fast => {1},
                            $crate::litei2cmaster::LiteI2CSpeedMode::FastPlus => {2}
                        };

                    //Set PHY speed mode
                    registers.
                    phy_speed_mode().write(|w| unsafe { w.bits(speed_mode_reg_setting) });

                    Self { registers, speed_mode }
                }

                pub fn free(self) -> $PACI2CMASTERX {
                    self.registers
                }
            }

            impl $crate::hal::i2c::ErrorType for $LITEI2CMASTER {
               type Error = $crate::litei2cmaster::LiteI2CError;
            }

            impl I2c<$crate::hal::i2c::SevenBitAddress> for $LITEI2CMASTER {

                fn transaction(
                    &mut self,
                    address: u8,
                    operations: &mut [$crate::hal::i2c::Operation<'_>],
                ) -> Result<(), $crate::litei2cmaster::LiteI2CError> {

                    if operations.len() == 0 {
                        return Ok(());
                    }

                    let mut operations_peekable_iter = operations.iter_mut().peekable();


                    //Activate master
                    self.registers.master_active().write(|w| unsafe { w.bits(1) });

                    //Flush RX buffer
                    while self.registers.master_status().read().rx_ready().bit() {
                        let _ = self.registers.master_rxtx().read().bits();
                    }
                    //Wait for TX ready
                    while !self.registers.master_status().read().tx_ready().bit() {
                    }

                    //Set slave address
                    self.registers.master_addr().write(|w| unsafe { w.bits(address as u32) });

                    let mut len_tx_buf : usize = 0;
                    let mut len_rx_buf : usize = 0;
                    let mut len_tx : u8 = 0;
                    let mut len_rx : u8 = 0;
                    let mut tx_buf : u32 = 0;
                    let mut write_next_read_op = false;

                    loop {

                        if let Some(operation) = operations_peekable_iter.next() {

                            write_next_read_op = false;

                            println!("New operation...");

                            match operation {
                                $crate::hal::i2c::Operation::Read(read_buffer) => {
                                    len_rx_buf = read_buffer.len();
                                },
                                $crate::hal::i2c::Operation::Write(write_buffer) => {

                                    len_tx_buf = write_buffer.len();

                                }

                            }

                            println!("len_rx_buf {} len_tx_buf {}", len_rx_buf, len_tx_buf);

                            if len_rx_buf > 255 || len_rx_buf > 255 {
                                println!("Operation too large! Exiting.");
                                break;
                            }

                            let mut tx_j : usize = 0;
                            let mut rx_j : usize = 0;

                            loop {

                                if len_tx_buf > (tx_j + 4) {
                                    len_tx = 5;
                                    len_rx = 0;
                                } else {
                                    len_tx = u8::try_from(len_tx_buf - tx_j).unwrap();

                                    if (len_rx_buf > (rx_j + 4)) {
                                        len_rx = 5;
                                    } else {
                                        len_rx = u8::try_from(len_rx_buf - rx_j).unwrap();
                                    }
                                }

                                println!("len_rx {} len_tx {}", len_rx, len_tx);

                                tx_buf = 0;

                                match operation {
                                    $crate::hal::i2c::Operation::Write(write_buffer) => {

                                        match len_tx {
                                        5 | 4 => {
                                            tx_buf |= (write_buffer[0 + tx_j] as u32) << 24_u32;
                                            tx_buf |= (write_buffer[1 + tx_j] as u32) << 16_u32;
                                            tx_buf |= (write_buffer[2 + tx_j] as u32) << 8_u32;
                                            tx_buf |= write_buffer[3 + tx_j] as u32;
                                            tx_j = tx_j.checked_add(4).unwrap();
                                        },
                                        3 => {
                                            tx_buf |= (write_buffer[0 + tx_j] as u32) << 16_u32;
                                            tx_buf |= (write_buffer[1 + tx_j] as u32) << 8_u32;
                                            tx_buf |= write_buffer[2 + tx_j] as u32;
                                            tx_j = tx_j.checked_add(3).unwrap();
                                        },
                                        2 =>  {
                                            tx_buf |= (write_buffer[0 + tx_j] as u32) << 8_u32;
                                            tx_buf |= write_buffer[1 + tx_j] as u32;
                                            tx_j = tx_j.checked_add(2).unwrap();
                                        },
                                        1 => {
                                            tx_buf |= write_buffer[0 + tx_j] as u32;
                                            tx_j = tx_j.checked_add(1).unwrap();
                                        }
                                        _ => { panic!("Invalid len_tx"); }
                                    };

                                    },
                                    _ => {}
                                }

                                //Write transfer settings
                                    self.registers
                                        .master_settings()
                                        .write(|w| unsafe { w.len_tx().bits(len_tx).len_rx().bits(len_rx).recover().bit(false) });

                                //Send data
                                self.registers
                                        .master_rxtx()
                                        .write(|w| unsafe { w.bits(tx_buf) });

                                while !self.registers.master_status().read().rx_ready().bit() {}

                                //Abort if NACK is received
                                if self.registers.master_status().read().nack().bit() {

                                    println!("Error - NACK Data received!");
                                    self.registers.master_active().write(|w| unsafe { w.bits(0) });

                                    return Err(LiteI2CError::NACK);

                                }

                                let rx_buf : u32 =  self.registers.master_rxtx().read().bits();


                                if len_rx_buf > 0 {

                                     match operation {

                                        $crate::hal::i2c::Operation::Read(read_buffer) => {

                                            println!("Current operation is read...directly setting data");

                                            match len_rx {
                                                5 | 4 => {
                                                    read_buffer[0 + rx_j] = (rx_buf >> 24_u32) as u8;
                                                    read_buffer[1 + rx_j] = (rx_buf >> 16_u32) as u8;
                                                    read_buffer[2 + rx_j] = (rx_buf >> 8_u32) as u8;
                                                    read_buffer[3 + rx_j] = rx_buf as u8;
                                                    rx_j = rx_j.checked_add(4).unwrap();
                                                },
                                                3 => {
                                                    read_buffer[0 + rx_j] = (rx_buf >> 16_u32) as u8;
                                                    read_buffer[1 + rx_j] = (rx_buf >> 8_u32) as u8;
                                                    read_buffer[2 + rx_j] = rx_buf as u8;
                                                    rx_j = rx_j.checked_add(3).unwrap();
                                                },
                                                2 =>  {
                                                    read_buffer[0 + rx_j] = (rx_buf >> 8_u32) as u8;
                                                    read_buffer[1 + rx_j] = rx_buf as u8;
                                                    rx_j = rx_j.checked_add(2).unwrap();
                                                },
                                                1 => {
                                                    read_buffer[0 + rx_j] = rx_buf as u8;
                                                    rx_j = rx_j.checked_add(1).unwrap();
                                                }
                                                _ => { panic!("Invalid len_rx"); }
                                            };
                                        },
                                        _ => { }
                                    }

                                }

                                if (tx_j >= len_tx_buf) || (rx_j >= len_rx_buf) {
                                    println!("Operation finished, going to next one.");
                                    break;
                                }

                            }

                        }
                        else {
                            println!("All operations processed.");
                            break;
                        }


                    }

                    //Deactivate master
                    self.registers.master_active().write(|w| unsafe { w.bits(0) });

                    Ok(())
                }
            }

        )+
    }
}
