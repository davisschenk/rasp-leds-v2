use crate::color::Color;
use crate::error::Result;

/// A struct which is capable of being treated as an LED strip
pub trait LedController {
    /// Return a mutable reference to a list of pixels.
    fn get_data(&mut self) -> &mut Vec<Color>;

    /// Return the number of pixels
    fn get_count(&self) -> usize;

    /// Display data on the device
    fn update(&mut self) -> Result<()>;

    /// Clear all leds without changing internal state
    fn clear(&mut self, state: bool) -> Result<()>;
}
