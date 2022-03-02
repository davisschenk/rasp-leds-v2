use std::iter::repeat;

use crate::Color;
use crate::controller::{Controller, LedController};
use crate::error::Result;
use crate::patterns::RunnablePattern;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Solid {
    color: Color
}

impl Solid {
    pub fn new(color: Color) -> Solid {
        Solid { color }
    }
}

impl RunnablePattern for Solid {
    fn tick_rate(&self) -> u64 {
        100
    }

    fn tick(&mut self, _tick: u64, controller: &mut Controller) -> Result<()> {
        let data = controller.get_data();
        data.iter_mut().for_each(|l| *l = self.color);
        controller.update()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AlternatingColor {
    count: u8,
    color: Color
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Alternating {
    colors: Vec<AlternatingColor>
}

impl Alternating {
    pub fn new(colors: Vec<AlternatingColor>) -> Alternating {
        Alternating { colors }
    }
}

impl RunnablePattern for Alternating {
    fn tick_rate(&self) -> u64 {
        100
    }

    fn tick(&mut self, _tick: u64, controller: &mut Controller) -> Result<()> {
        let data = controller.get_data();

        self.colors
            .iter()
            .flat_map(|c| repeat(c.color).take(c.count as usize))
            .cycle()
            .zip(data.iter_mut())
            .for_each(|(c, l)| *l = c);

        controller.update()
    }
}
