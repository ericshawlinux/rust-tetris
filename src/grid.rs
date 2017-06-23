use std::cmp;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

use block::Block;
use block::Point;
use block::Plot;

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
        'validation:
        for point in &new_plot {

            // collision with self is ok, skip
            for current_point in &block.plot {
                if (point.x, point.y) == (current_point.x, current_point.y) {
                    continue 'validation;
                }
            }
            // check grid boundaries
            if point.x < 0 || point.y < 0 {
                println!("Too low");
                return false;
            }
            if point.x >= GRID_WIDTH as i32 || point.y >= GRID_HEIGHT as i32 {
                println!("Too high");
                return false;
            }
            // check for collision
            if self.cells[point.y as usize][point.x as usize] != Color::RGB(0, 0, 0) {
                println!("there's another block here");
                return false;
            }
        }

        // erase old block
        for point in &block.plot {
            self.cells[point.y as usize][point.x as usize] = Color::RGB(0, 0, 0);
        }

        // update block plot
        block.plot = new_plot;

        // update grid
        for point in &block.plot {
            self.cells[point.y as usize][point.x as usize] = block.color;
        }

        true
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
}