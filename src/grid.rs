/* grid.rs
 *  
 * Created by Eric Shaw Jr. on 2017-06-18
 * Copyright (c) 2017, 2018 Eric Shaw Jr.
 * 
 * This file is part of rustlang-game. It is subject to the license terms
 * in the LICENSE file found in the top-level directory of this distribution.
 * No part of rustlang-game, including this file, may be copied, modified, propagated,
 * or distributed except according to the terms contained in the LICENSE file.
 */

use std::f64;
use sdl2::pixels::Color;

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
        let new_plot = Block::translate_plot(&block.plot, change);

        // validate boundaries and collisions
        if !block.validate_plot(&mut self.cells, &new_plot) {
            return false;
        }

        block.finalize_plot(&mut self.cells, new_plot, change);

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

        // try no each of these translations, starting with none, until one works
        let trials: [Point; 7] = [
            direction::NONE,
            direction::UP,
            direction::UP + direction::UP,
            direction::LEFT,
            direction::LEFT + direction::LEFT,
            direction::RIGHT,
            direction::RIGHT + direction::RIGHT,
        ];

        for trial_direction in &trials {

            let trial = Block::translate_plot(&rotated_plot, *trial_direction);

            if block.validate_plot(&mut self.cells, &trial) {
                block.finalize_plot(&mut self.cells, trial, *trial_direction);
                return;
            }
        }
    }
}