mod controller;
pub use controller::*;

#[cfg(all(feature = "simulate", feature = "hardware"))]
compile_error!("Cannot use simulated controller and hardware controller at the same time!");

#[cfg(feature = "simulate")]
mod simulate;
#[cfg(feature = "simulate")]
pub use simulate::*;

#[cfg(feature = "hardware")]
mod hardware;
#[cfg(feature = "hardware")]
pub use hardware::*;
