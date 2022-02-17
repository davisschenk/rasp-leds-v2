use std::{thread, time::Duration};

use rasp_leds_v2::*;

#[test]
fn test_runner() {
    #[cfg(feature = "simulate")]
    let mut runner = LedRunner::new(150, 25);

    #[cfg(feature = "hardware")]
    let mut runner = Runner::new(300, 18, 255);

    runner.start();

    let pattern = MovingDot {
        tick_rate: 100,
        tick_cycle: 150,
        color: Color::RGB(255, 0, 0),
    };

    runner.run_pattern(Pattern::MovingDot(pattern));
    thread::sleep(Duration::from_secs(10));

    println!("changing pattern");

    let pattern = MovingDot {
        tick_rate: 100,
        tick_cycle: 150,
        color: Color::RGB(255, 255, 0),
    };

    runner.run_pattern(Pattern::MovingDot(pattern));
    thread::sleep(Duration::from_secs(10));
}
