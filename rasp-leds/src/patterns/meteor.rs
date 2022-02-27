use crate::color::Color;
use crate::controller::{Controller, LedController};
use crate::error::{LedError, Result};
use crate::patterns::RunnablePattern;
use rand::Rng;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Meteor {
    pub tick_rate: u64,
    pub tick_cycle: u64,
    pub color: Color,
    pub random_decay: bool,
    pub decay: u8,
    pub size: u8,
}

impl RunnablePattern for Meteor {
    fn init(&mut self, controller: &mut Controller) -> Result<()> {
        self.tick_cycle = controller.get_count() as u64 * 2;
        Ok(())
    }

    fn tick_rate(&self) -> u64 {
        self.tick_rate
    }

    fn tick_cycle(&self) -> Option<u64> {
        Some(self.tick_cycle)
    }

    fn tick(&mut self, tick: u64, controller: &mut Controller) -> Result<()> {
        let mut rng = rand::thread_rng();
        let count = controller.get_count();
        let data = controller.get_data();

        for led in data.iter_mut() {
            if !self.random_decay || rng.gen::<bool>() {
                *led = led.fade_to_black(self.decay);
            }
        }

        for j in 0..self.size {
            if ((tick as isize) - (j as isize) < count as isize)
                && (tick as isize - j as isize >= 0)
            {
                data[tick as usize - j as usize] = self.color;
            }
        }
        controller.update()
    }
}
