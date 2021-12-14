use color::*;
use controller::*;
use std::{thread::sleep, time::Duration};

fn main() {
    #[cfg(feature = "simulate")]
    let mut c = Controller::new(150, 25);

    #[cfg(feature = "hardware")]
    let mut c = Controller::new(150, 18, 255);

    let colors = vec![
        Color::RGB(255, 0, 0),
        Color::RGB(0, 255, 0),
        Color::RGB(0, 0, 255),
    ];

    for (colors, color) in c
        .get_data()
        .chunks_mut(5)
        .zip(colors.iter().cycle())
    {
        for c in colors {
            *c = *color;
        }
    }

    loop {
        c.get_data().rotate_left(2);
        c.update().unwrap();
        sleep(Duration::from_millis(100));

        c.get_data().rotate_right(4);
        c.update().unwrap();
        sleep(Duration::from_millis(100));
    }
}
