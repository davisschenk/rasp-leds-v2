#[cfg(feature = "patterns")]
mod pattern;

#[cfg(feature = "patterns")]
pub use pattern::*;

#[cfg(feature = "patterns")]
pub mod moving_dot;

#[cfg(feature = "patterns")]
pub use moving_dot::*;

#[cfg(feature = "patterns")]
pub mod rainbow;

#[cfg(feature = "patterns")]
pub use rainbow::*;
