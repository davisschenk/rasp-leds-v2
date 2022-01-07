use crate::color::Color;
use crate::controller::{Controller, LedController};
use crate::patterns::Pattern;
use anyhow::Result;

pub struct MovingDot {
    pub tick_rate: u64,
    pub tick_cycle: u64,
    pub color: Color,
}

impl Pattern for MovingDot {
    fn tick_rate(&self) -> u64 {
        self.tick_rate
    }

    fn tick_cycle(&self) -> Option<u64> {
        Some(self.tick_cycle)
    }

    fn tick(&mut self, tick: u64, controller: &mut Controller) -> Result<()> {
        let count = controller.get_count();
        let leds = controller.get_data();

        leds[tick as usize] = self.color;

        if tick > 0 {
            leds[(tick - 1) as usize] = Color::RGB(0, 0, 0);
        } else {
            leds[count - 1] = Color::RGB(0, 0, 0);
        }

        controller.update()
    }
}
