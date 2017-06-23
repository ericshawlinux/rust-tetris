use rand;
use rand::Rng;
use sdl2::pixels::Color;
use grid::Direction;

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
#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// represents a set of points on the grid
pub type Plot = [Point; 4];

pub struct Block {
    pub plot:   Plot,
    pub color:  Color,
    offset:     Point,
    models:     &'static Models,
    rotation:   usize,
}

impl Block {

    pub fn new() -> Block {

        Block {
            plot: [Point { x: 5, y: 0 }, Point { x: 5, y: 1 }, Point { x: 5, y: 2 }, Point { x: 6, y: 2 }],
            color: Color::RGB(255, 165, 0),
            offset: Point { x: 5, y: 0},
            models: &L_SHAPE,
            rotation: 0,
        }
    }

    pub fn get_rotation_plot(&mut self) -> Plot {

        let model = &self.models[self.rotation];
        let mut plot: Plot = model.clone();

        for p in 0..model.len() {
            let point = model[p];

            plot[p] = Point { x: point.x + self.offset.x, y: point.y + self.offset.y };
        }

        plot
    }

    pub fn increment_rotation(&mut self) {

        if self.rotation >= MAX_ROTATION {
            self.rotation = 0;
        }
        else {
            self.rotation += 1;
        }
    }

    pub fn increment_offset(&mut self, direction: Direction) {
        
        if direction == Direction::Left {
            self.offset.x -= 1;
        }
        else if direction == Direction::Right {
            self.offset.x += 1;
        }
        else if direction == Direction::Down {
            self.offset.y += 1;
        }
    }
}

const MAX_ROTATION: usize = 3;

type Models = [Plot; 4];

static L_SHAPE: Models = [
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 0, y: 2 }, Point { x: 1, y: 2 }],
    [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 0, y: 1 }],
    [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }],
    [Point { x: 2, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
];