use crate::color::Color;
use anyhow::Result;

pub trait LedController {
    fn get_data(&mut self) -> &mut Vec<Color>;
    fn get_count(&self) -> usize;
    fn update(&mut self) -> Result<()>;
}
