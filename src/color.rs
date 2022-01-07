#[derive(Clone, Copy, Debug)]
pub enum Color {
    RGB(u8, u8, u8),
}

impl Color {
    pub fn to_int(&self) -> u32 {
        match self {
            Color::RGB(r, g, b) => (*r as u32) << 16 | (*g as u32) << 8 | *b as u32,
        }
    }

    pub fn to_arr(&self) -> [u8; 4] {
        match self {
            Color::RGB(r, g, b) => [*b, *g, *r, 0],
        }
    }

    pub fn fade_to_black(&self, amt: u8) -> Color {
        match self {
            Color::RGB(r, g, b) => {
                Color::RGB(
                    if *r <= 10 { 0 } else {*r - (*r as f64 * amt as f64 / 255.0) as u8},
                    if *b <= 10 { 0 } else {*b - (*b as f64 * amt as f64 / 255.0) as u8},
                    if *g <= 10 { 0 } else {*g - (*g as f64 * amt as f64 / 255.0) as u8},
                )
            }
        }
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        self.to_int()
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        self.to_arr()
    }
}

pub fn wheel(position: u8) -> Color {
    if position < 85 {
        Color::RGB(position * 3, 255 - position * 3, 0)
    } else if position < 170 {
        Color::RGB(255 - (position - 85) * 3, 0, (position - 85) * 3)
    } else {
        Color::RGB(0, (position - 170) * 3, 255 - (position - 170) * 3)
    }
}
