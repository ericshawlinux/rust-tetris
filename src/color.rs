use rand;
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
    Empty,
}

pub fn next_color() -> Color {
    let n = rand::thread_rng().gen_range(1, 8);

    match n {
        1 => Color::Red,
        2 => Color::Orange,
        3 => Color::Yellow,
        4 => Color::Green,
        5 => Color::Blue,
        6 => Color::Indigo,
        _ => Color::Violet,
    }
}
