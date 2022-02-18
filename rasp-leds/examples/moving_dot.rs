use rasp_leds_hal::*;

use std::{thread::sleep, time::Duration};

fn main() {
    #[cfg(feature = "simulate")]
    let mut runner: LedRunner = LedRunner::new(150, 25);

    #[cfg(feature = "hardware")]
    let mut runner = Runner::new(300, 18, 255);

    let pattern = MovingDot {
        tick_rate: 100,
        tick_cycle: 150,
        color: Color::RGB(255, 0, 0),
    };

    runner.run_pattern(Pattern::MovingDot(pattern));
    runner.start();
    sleep(Duration::from_secs(5));

    let pattern = MovingDot {
        tick_rate: 100,
        tick_cycle: 150,
        color: Color::RGB(255, 0, 255),
    };
    runner.run_pattern(Pattern::MovingDot(pattern));
}
