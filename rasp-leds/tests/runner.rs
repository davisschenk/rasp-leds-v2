use std::{thread, time::Duration};
use serde_json::json;
use rasp_leds_hal::*;

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

    let p = Pattern::Meteor(pattern);
    println!("{:?}", serde_json::to_string(&p).unwrap());

    runner.run_pattern(p);
    thread::sleep(Duration::from_secs(10));

    runner.off();
    thread::sleep(Duration::from_secs(3));
    runner.on();
    thread::sleep(Duration::from_secs(3));
    runner.power();
    thread::sleep(Duration::from_secs(3));
    runner.power();
    thread::sleep(Duration::from_secs(10));
}
