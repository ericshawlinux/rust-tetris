use rand;
use rand::Rng;
use sdl2::pixels::Color;
use point::Point;
use point::Plot;

/*
    Color::RGB(0, 255, 255),
    Color::RGB(15, 100, 255),
    Color::RGB(255, 165, 0),
    Color::RGB(255, 255, 0),
    Color::RGB(0, 255, 0),
    Color::RGB(210, 0, 128),
    Color::RGB(255, 0, 0),
    Color::RGB(0, 0, 0),
*/

pub struct Block {
    pub plot:   Plot,
    pub center: Point,
    pub color:  Color,
}

impl Block {

    pub fn new() -> Block {

        Block {
            plot: [Point { x: 5, y: 0 }, Point { x: 5, y: 1 }, Point { x: 5, y: 2 }, Point { x: 6, y: 2 }],
            center: Point { x: 5, y: 1 },
            color: Color::RGB(255, 165, 0),
        }
    }
}