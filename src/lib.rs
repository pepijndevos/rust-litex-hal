#![no_std]

#[cfg(feature = "gpio")]
pub mod gpio;
pub mod spi;
pub mod timer;
pub mod uart;

pub use embedded_hal as hal;
pub use nb;

pub mod prelude {
    pub use embedded_hal::prelude::*;
}
