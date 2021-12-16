use crate::controller::Controller;
use anyhow::Result;
use enum_dispatch::enum_dispatch;

use crate::patterns::*;

#[enum_dispatch(Pattern)]
pub enum Patterns {
    MovingDot(MovingDot),
}

#[enum_dispatch]
pub trait Pattern {
    fn init(&self, _controller: &mut Controller) -> Result<()> {
        Ok(())
    }
    fn tick_rate(&self) -> u64;
    fn tick_cycle(&self) -> Option<u64> {
        None
    }

    fn start_tick(&mut self, raw_tick: u64, leds: &mut Controller) -> Result<()> {
        match self.tick_cycle() {
            Some(cycle) => self.tick(raw_tick % cycle, leds),
            None => self.tick(raw_tick, leds),
        }
    }

    fn tick(&mut self, tick: u64, controller: &mut Controller) -> Result<()>;
}
