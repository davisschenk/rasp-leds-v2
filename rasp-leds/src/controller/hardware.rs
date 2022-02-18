use super::LedController;
use crate::color::*;
use rs_ws281x::{ChannelBuilder, Controller as RaspController, ControllerBuilder, StripType};

pub struct Controller {
    data: Vec<Color>,
    count: usize,
    controller: RaspController,
}

impl Default for Controller {
    fn default() -> Self {
        Self::new(300, 18, 255)
    }
}

impl Controller {
    pub fn new(count: usize, pin: i32, brightness: u8) -> Self {
        let channel = ChannelBuilder::new()
            .pin(pin)
            .count(count as i32)
            .brightness(brightness)
            .strip_type(StripType::Ws2812)
            .build();

        let controller = ControllerBuilder::new()
            .freq(800_000)
            .dma(10)
            .channel(0, channel)
            .build()
            .expect("Failed to create controller");

        Self {
            data: vec![Color::RGB(0, 0, 0); count],
            count,
            controller,
        }
    }
}

impl LedController for Controller {
    fn get_data(&mut self) -> &mut Vec<Color> {
        &mut self.data
    }

    fn update(&mut self) -> anyhow::Result<()> {
        for (led, color) in self.controller.leds_mut(0).iter_mut().zip(self.data.iter()) {
            *led = color.to_arr();
        }

        self.controller.render().unwrap();
        Ok(())
    }

    fn get_count(&self) -> usize {
        self.count
    }

    fn clear(&self, state: bool) -> anyhow::Result<()> {
        for led in self.controller.leds_mut(0).iter_mut() {
            *led = &[0,0,0,0]
        }

        if state {
            self.data.fill(Color::RGB(0,0,0))
        }

        self.controller.render().unwrap();
        Ok(())
    }
}
