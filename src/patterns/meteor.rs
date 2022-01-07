use crate::color::Color;
use crate::controller::{Controller, LedController};
use crate::patterns::Pattern;
use anyhow::Result;
use rand::Rng;

pub struct Meteor {
    pub tick_rate: u64,
    pub tick_cycle: u64,
    pub color: Color,
    pub random_decay: bool,
    pub decay: u8,
    pub size: u8
}

impl Pattern for Meteor {
    fn init(&mut self, controller: &mut Controller) -> Result<()>{
        self.tick_cycle = controller.get_count() as u64 + self.size as u64;
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

        for i in tick..tick+self.size as u64 {
            if i < count as u64{
                    data[i as usize] = self.color;
            }
        }

        // for j in 0..self.size {
        //     if count as u64 > tick - j as u64 {
        //         data[(tick - j as u64) as usize] = self.color;
        //     }
        // }

        controller.update()
    }
}
