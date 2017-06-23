use rand;
use rand::Rng;
use sdl2::pixels::Color;
use grid::Direction;
use grid::GRID_HEIGHT;
use grid::GRID_WIDTH;

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

        let (models, color) = next_block();
        let offset = Point { x: 5, y: 0 };
        let mut plot = models[0];
        
        for p in 0..models[0].len() {
            let point = plot[p];
            plot[p] = Point { x: point.x + offset.x, y: point.y + offset.y };
        }

        Block {
            plot: plot,
            color: color,
            offset: offset,
            models: models,
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
    [Point { x: 2, y: 1 }, Point { x: 0, y: 2 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 }],
];

static I_SHAPE: Models = [
    [Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }, Point { x: 1, y: 3 }],
    [Point { x: 0, y: 2 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 }, Point { x: 3, y: 2 }],
    [Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }, Point { x: 1, y: 3 }],
    [Point { x: 0, y: 2 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 }, Point { x: 3, y: 2 }],
];

fn next_block() -> (&'static Models, Color) {
    let n = rand::thread_rng().gen_range(1, 3);

    match n {
        1 => (&L_SHAPE, Color::RGB(0, 255, 255)),
        _ => (&I_SHAPE, Color::RGB(15, 100, 255)),
    }
}