use sdl2::pixels::Color;

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

                Point { x: x, y: y }
            };
        }

        // validate boundaries and collisions
        if !self.move_is_valid(block, &new_plot) {
            return false;
        }

        // erase old block
        for point in &block.plot {
            self.cells[point.y as usize][point.x as usize] = Color::RGB(0, 0, 0);
        }

        // update block plot, grid, and offset
        block.plot = new_plot;
        self.place(block);

        block.increment_offset(direction);

        true
    }

    pub fn rotate_block(&mut self, block: &mut Block) {
        
        let new_plot = block.get_rotation_plot();

        // validate boundaries and collisions
        if !self.move_is_valid(block, &new_plot) {
            return;
        }
        block.increment_rotation();

        // erase old block
        for point in &block.plot {
            self.cells[point.y as usize][point.x as usize] = Color::RGB(0, 0, 0);
        }

        // update block plot
        block.plot = new_plot;
        self.place(block);
    }

    pub fn move_is_valid(&self, block: &Block, new_plot: &Plot) -> bool {

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
}

#[derive(PartialEq)]
pub enum Direction {
    Down,
    Left,
    Right,
}