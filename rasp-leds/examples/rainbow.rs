use rasp_leds_hal::{Controller, RainbowCycle, RunnablePattern};
use std::{thread::sleep, time::Duration};

fn main() {
    #[cfg(feature = "simulate")]
    let mut c = Controller::new(150, 25);

    #[cfg(feature = "hardware")]
    let mut c = Controller::new(300, 18, 255);

    let mut pattern = RainbowCycle::new(100);

    pattern.init(&mut c).unwrap();

    for i in 0..u64::MAX {
        pattern.start_tick(i, &mut c).unwrap();
        sleep(Duration::from_millis(pattern.tick_rate()))
    }
}
