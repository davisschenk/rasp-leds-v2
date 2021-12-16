#[derive(Clone, Copy)]
pub enum Color {
    RGB(u8, u8, u8),
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        match self {
            Color::RGB(r, g, b) => (r as u32) << 16 | (g as u32) << 8 | b as u32,
        }
    }
}

impl Into<[u8; 4]> for &Color {
    fn into(self) -> [u8; 4] {
        match self {
            Color::RGB(r, g, b) => [*b, *g, *r, 0],
        }
    }
}
