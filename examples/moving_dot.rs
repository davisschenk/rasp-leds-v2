use rasp_leds_v2::{Color, Controller, MovingDot, Pattern};
use std::{thread::sleep, time::Duration};

fn main() {
    #[cfg(feature = "simulate")]
    let mut c = Controller::new(150, 25);

    #[cfg(feature = "hardware")]
    let mut c = Controller::new(150, 18, 255);

    let mut pattern = MovingDot {
        tick_rate: 100,
        tick_cycle: 150,
        color: Color::RGB(255, 0, 0),
    };

    for i in 0..u64::MAX {
        pattern.start_tick(i, &mut c).unwrap();
        sleep(Duration::from_millis(pattern.tick_rate()))
    }
}
