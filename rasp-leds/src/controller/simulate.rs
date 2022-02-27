use std::thread;

#[cfg(feature = "simulate")]
use minifb::{Key, Window, WindowOptions};

use super::LedController;
use crate::color::Color;
use crate::error::Result;
use std::sync::{mpsc, Arc, Mutex};

// const CELL_SIZE: usize = 25;

pub struct Controller {
    data: Vec<Color>,
    buffer: Arc<Mutex<Vec<u32>>>,
    count: usize,
    update_channel: mpsc::Sender<()>,
    cell_size: usize,
}

impl Controller {
    pub fn new(count: usize, cell_size: usize) -> Self {
        let height: usize = cell_size;
        let width: usize = cell_size * count;
        let buffer = vec![0; width * height];
        let (tx, rx) = mpsc::channel();

        let controller = Self {
            count,
            data: vec![Color::RGB(0, 0, 0); count],
            buffer: Arc::new(Mutex::new(buffer)),
            update_channel: tx,
            cell_size,
        };

        let buff = controller.buffer.clone();
        thread::spawn(move || {
            let mut window =
                Window::new("Simulated LEDS", width, height, WindowOptions::default()).unwrap();
            window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

            while window.is_open() && !window.is_key_down(Key::Escape) {
                if rx.try_recv().is_ok() {
                    let data = buff.lock().unwrap();
                    window.update_with_buffer(&*data, width, height).unwrap();
                } else {
                    window.update();
                }
            }

            std::process::exit(0);
        });

        controller
    }
}

impl LedController for Controller {
    fn get_data(&mut self) -> &mut Vec<Color> {
        &mut self.data
    }

    fn get_count(&self) -> usize {
        self.count
    }

    fn update(&mut self) -> Result<()> {
        let mut buffer = self.buffer.lock().unwrap();

        for (index, i) in buffer.iter_mut().enumerate() {
            *i = self.data[index / self.cell_size % self.count].into();
        }

        self.update_channel.send(()).unwrap();

        Ok(())
    }

    fn clear(&mut self, state: bool) -> Result<()> {
        let mut buffer = self.buffer.lock().unwrap();

        for i in buffer.iter_mut() {
            *i = 0
        }

        if state {
            self.data.fill(Color::RGB(0, 0, 0))
        }

        self.update_channel.send(()).unwrap();

        Ok(())
    }
}
