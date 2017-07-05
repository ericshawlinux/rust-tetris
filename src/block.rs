use rand;
use rand::Rng;
use sdl2::pixels::Color;
use std::ops::Sub;
use std::ops::Add;

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

// represents a point on the grid
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// represents a set of points on the grid
pub type Plot = [Point; 4];

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