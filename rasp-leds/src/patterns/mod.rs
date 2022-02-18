use crate::controller::Controller;
use anyhow::Result;
use enum_dispatch::enum_dispatch;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

pub mod moving_dot;
pub use moving_dot::*;

pub mod rainbow;
pub use rainbow::*;

pub mod meteor;
pub use meteor::*;

#[derive(Debug, Clone)]
#[enum_dispatch(RunnablePattern)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case", tag = "pattern"))]
pub enum Pattern {
    MovingDot(MovingDot),
    RainbowCycle(RainbowCycle),
    Meteor(Meteor),
}

#[enum_dispatch]
pub trait RunnablePattern {
    /// This function is called when a pattern is first ran IE on tick 0.
    fn init(&mut self, _controller: &mut Controller) -> Result<()> {
        Ok(())
    }

    /// Should return the rate in hz for ticks to be ran
    fn tick_rate(&self) -> u64;

    /// Returns an optional value which controls the value given to the tick function, if none then its just the raw tick value, otherwise its raw_tick % tick_cycle
    fn tick_cycle(&self) -> Option<u64> {
        None
    }

    /// Run a tick
    fn start_tick(&mut self, raw_tick: u64, leds: &mut Controller) -> Result<()> {
        match self.tick_cycle() {
            Some(cycle) => self.tick(raw_tick % cycle, leds),
            None => self.tick(raw_tick, leds),
        }
    }

    /// User function for actually implementing tick behavior.
    fn tick(&mut self, tick: u64, controller: &mut Controller) -> Result<()>;
}
