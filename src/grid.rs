use std::f64;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

use block::Block;
use point::Point;
use point::Plot;
use point::direction;

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

    pub fn move_block(&mut self, block: &mut Block, change: Point) -> bool {
        
        // create a new plot based on the change parameter
        let new_plot = Grid::translate_plot(&block.plot, change);

        // validate boundaries and collisions
        if !self.validate_plot(block, &new_plot) {
            return false;
        }

        self.finalize_plot(block, new_plot, change);

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

        // try no translation
        if self.validate_plot(block, &rotated_plot) {
            self.finalize_plot(block, rotated_plot, direction::NONE);
            return;
        }

        // otherwise, try these
        let trials: [Point; 6] = [
            direction::UP,
            direction::UP + direction::UP,
            direction::LEFT,
            direction::LEFT + direction::LEFT,
            direction::RIGHT,
            direction::RIGHT + direction::RIGHT,
        ];

        for trial_direction in &trials {

            let trial = Grid::translate_plot(&rotated_plot, *trial_direction);

            if self.validate_plot(block, &trial) {
                self.finalize_plot(block, trial, *trial_direction);
                return;
            }
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

    fn finalize_plot(&mut self, block: &mut Block, new_plot: Plot, change: Point) {

        // erase old block
        for point in &block.plot {
            self.cells[point.y as usize][point.x as usize] = Color::RGB(0, 0, 0);
        }

        // update block plot
        block.plot = new_plot;
        block.center = block.center + change;

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