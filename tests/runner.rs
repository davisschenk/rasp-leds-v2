use std::{thread, time::Duration};

use rasp_leds_v2::*;

#[test]
fn test_runner() {
    #[cfg(feature = "simulate")]
    let mut runner = LedRunner::new(150, 25);

    #[cfg(feature = "hardware")]
    let mut runner = Runner::new(300, 18, 255);

    runner.start();

    let pattern = Meteor {
        tick_rate: 100,
        tick_cycle: 150,
        color: Color::RGB(255, 0, 255),
        random_decay: true,
        decay: 32,
        size: 18,
    };

    runner.run_pattern(Pattern::Meteor(pattern));
    thread::sleep(Duration::from_secs(100))
}
