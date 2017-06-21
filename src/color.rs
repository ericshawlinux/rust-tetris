use rand;
use rand::Rng;
use sdl2::pixels;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Cyan,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red,
    Empty,
}

impl Color {
    pub fn to_rgb(self) -> pixels::Color {
        match self {
            Color::Cyan     => pixels::Color::RGB(0, 255, 255),
            Color::Blue     => pixels::Color::RGB(15, 100, 255),
            Color::Orange   => pixels::Color::RGB(255, 165, 0),
            Color::Yellow   => pixels::Color::RGB(255, 255, 0),
            Color::Green    => pixels::Color::RGB(0, 255, 0),
            Color::Purple   => pixels::Color::RGB(210, 0, 128),
            Color::Red      => pixels::Color::RGB(255, 0, 0),
            Color::Empty    => pixels::Color::RGB(0, 0, 0),
        }
    }
}

pub fn next_color() -> Color {
    let n = rand::thread_rng().gen_range(1, 8);

    match n {
        1 => Color::Cyan,
        2 => Color::Blue,
        3 => Color::Orange,
        4 => Color::Yellow,
        5 => Color::Green,
        6 => Color::Purple,
        7 => Color::Red,
        _ => panic!("no such non-empty color"),
    }
}