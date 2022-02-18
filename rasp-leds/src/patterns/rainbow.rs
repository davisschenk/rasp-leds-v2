use crate::controller::{Controller, LedController};
use crate::patterns::RunnablePattern;
use anyhow::Result;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RainbowCycle {
    pub tick_rate: u64,
}

impl RainbowCycle {
    pub fn new(tick_rate: u64) -> RainbowCycle {
        RainbowCycle { tick_rate }
    }
}

impl RunnablePattern for RainbowCycle {
    fn tick_rate(&self) -> u64 {
        self.tick_rate
    }

    fn tick_cycle(&self) -> Option<u64> {
        Some(256 * 5)
    }

    fn tick(&mut self, tick: u64, controller: &mut Controller) -> Result<()> {
        let count = controller.get_count();
        let leds = controller.get_data();

        for i in 0..leds.len() {
            leds[i] = crate::color::wheel(
                (((i as f64 * 256.0 / count as f64) as u64 + tick) & 255) as u8,
            );
        }

        let _ = controller.update();

        Ok(())
    }
}
