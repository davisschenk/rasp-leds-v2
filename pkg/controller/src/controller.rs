use anyhow::Result;
use color::Color;

pub trait LedController {
    // fn new(size: usize) -> Self;
    fn get_data(&mut self) -> &mut Vec<Color>;
    fn get_count(&self) -> usize;
    fn update(&mut self) -> Result<()>;
}
