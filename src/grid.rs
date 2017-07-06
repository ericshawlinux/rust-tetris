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

        let mut rotated_plot: Plot = [Point { x: 0, y: 0 }; 4];

        for p in 0..block.plot.len() {
            // translate the point to origin
            let cpoint = block.plot[p] - block.center;
            // do a 90 degree rotation and translate it back
            rotated_plot[p] = Point { x: -cpoint.y, y: cpoint.x } + block.center;
        }

        let (left, right, down, up) = (
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: -1 },
        );

        // no translation
        if self.validate_plot(block, &rotated_plot) {
            self.finalize_plot(block, rotated_plot, None);
            return;
        }

        // left once
        let left_plot = Grid::translate_plot(&rotated_plot, left);
        
        if self.validate_plot(block, &left_plot) {
            self.finalize_plot(block, left_plot, Some(Direction::Left));
            return;
        }

        // moved left twice
        let left_plot = Grid::translate_plot(&left_plot, left);

        if self.validate_plot(block, &left_plot) {
            // this will make the block.center incorrect. todo: stop using Direction enum.
            // instead, just use a point that describes the change
            self.finalize_plot(block, left_plot, Some(Direction::Left));
            return;
        }

        // moved right once
        let right_plot = Grid::translate_plot(&rotated_plot, right);

        if self.validate_plot(block, &right_plot) {
            self.finalize_plot(block, right_plot, Some(Direction::Right));
            return;
        }

        // moved right twice
        let right_plot = Grid::translate_plot(&right_plot, right);

        if self.validate_plot(block, &right_plot) {
            self.finalize_plot(block, right_plot, Some(Direction::Right));
            return;
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

    // used when rotating to try and shift the block a little if necessary
    fn translate_plot(plot: &Plot, change: Point) -> Plot {

        let mut result: Plot = *plot;

        for p in 0..plot.len() {
            result[p] = plot[p] + change;
        }

        result
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