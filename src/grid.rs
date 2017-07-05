use std::f64;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

use block::Block;
use block::Point;
use block::Plot;

pub const ROTATION: f64 = 1.57;

pub const GRID_HEIGHT:  usize = 20;
pub const GRID_WIDTH:   usize = 10;

pub type GridArray = [[Color; GRID_WIDTH]; GRID_HEIGHT];

pub struct Grid {
    cells: GridArray,
}

impl Grid {

    pub fn new() -> Grid {
        Grid {
            cells: [[Color::RGB(0, 0, 0); GRID_WIDTH]; GRID_HEIGHT],
        }
    }
    
    pub fn get_cells(&self) -> &GridArray {
        &self.cells
    }

    pub fn place(&mut self, block: &Block) -> &mut Grid {

        for point in &block.plot {
            self.cells[point.y as usize][point.x as usize] = block.color;
        }

        self
    }

    pub fn move_block(&mut self, block: &mut Block, direction: Direction) -> bool {
        
        // create a new plot based on the direction we're moving
        let mut new_plot = block.plot;

        for p in 0..block.plot.len() {
            new_plot[p] = {
                let x = block.plot[p].x;
                let y = block.plot[p].y;

                let (x, y) = match direction {
                    Direction::Left     => (x - 1, y),
                    Direction::Right    => (x + 1, y),
                    Direction::Down     => (x, y + 1),
                };

                Point {x: x, y: y}
            };
        }

        // validate boundaries and collisions
        if !self.validate_plot(block, &new_plot) {
            return false;
        }

        self.finalize_plot(block, new_plot, Some(direction));

        true
    }

    pub fn rotate_block(&mut self, block: &mut Block) {

        // translate the plot so that the center point is at origin

        let mut centered_plot: Plot = [Point { x: 0, y: 0 }; 4];
        
        for p in 0..block.plot.len() {
            centered_plot[p] = block.plot[p] - block.center;
        }

        // rotate the plot and return to former position

        let mut rotated_plot: Plot = centered_plot;

        for p in 0..centered_plot.len() {
            let (x, y) = (centered_plot[p].x, centered_plot[p].y);
            rotated_plot[p] = Point { x: -y, y: x } + block.center;
        }

        if self.validate_plot(block, &rotated_plot) {
            self.finalize_plot(block, rotated_plot, None)
        }

    }

    fn validate_plot(&self, block: &mut Block, new_plot: &Plot) -> bool {
        'validation:
        for point in new_plot {

            // collision with self is ok, skip
            for current_point in &block.plot {
                if (point.x, point.y) == (current_point.x, current_point.y) {
                    continue 'validation;
                }
            }
            // check grid boundaries
            if point.x < 0 || point.y < 0 {
                println!("Out of bounds");
                return false;
            }
            if point.x >= GRID_WIDTH as i32 || point.y >= GRID_HEIGHT as i32 {
                println!("Out of bounds");
                return false;
            }
            // check for collision
            if self.cells[point.y as usize][point.x as usize] != Color::RGB(0, 0, 0) {
                println!("Collision");
                return false;
            }
        }

        true
    }

    fn finalize_plot(&mut self, block: &mut Block, new_plot: Plot, direction: Option<Direction>) {

        // erase old block
        for point in &block.plot {
            self.cells[point.y as usize][point.x as usize] = Color::RGB(0, 0, 0);
        }

        // update block plot
        block.plot = new_plot;

        if let Some(direction) = direction {
            let change = Direction::get_point_difference(direction);
            block.center = block.center + change;
        }

        // update grid
        for point in &block.plot {
            self.cells[point.y as usize][point.x as usize] = block.color;
        }
    }
}

pub enum Direction {
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_keycode(keycode: Keycode) -> Option<Direction> {
        
        match keycode {
            Keycode::Left => Some(Direction::Left),
            Keycode::Right => Some(Direction::Right),
            Keycode::Down => Some(Direction::Down),
            _ => None
        }
    }

    pub fn get_point_difference(direction: Direction) -> Point {
        
        match direction {
            Direction::Left =>  Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1,  y: 0 },
            Direction::Down =>  Point { x: 0,  y: 1 },
        }
    }
}