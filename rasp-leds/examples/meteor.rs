use rasp_leds_v2::{Color, Controller, Meteor, RunnablePattern};
use std::{thread::sleep, time::Duration};

fn main() {
    #[cfg(feature = "simulate")]
    let mut c = Controller::new(150, 25);

    #[cfg(feature = "hardware")]
    let mut c = Controller::new(300, 18, 255);

    let mut pattern = Meteor {
        tick_rate: 100,
        tick_cycle: 10,
        color: Color::RGB(255, 0, 255),
        random_decay: true,
        decay: 32,
        size: 18,
    };

    pattern.init(&mut c).unwrap();

    for i in 0..u64::MAX {
        pattern.start_tick(i, &mut c).unwrap();
        sleep(Duration::from_millis(pattern.tick_rate()))
    }
}
