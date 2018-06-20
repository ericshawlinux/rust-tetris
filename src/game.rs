/* game.rs
 *  
 * Created by Eric Shaw Jr. on 2017-06-20
 * Copyright (c) 2017, 2018 Eric Shaw Jr.
 * 
 * This file is part of rustlang-game. It is subject to the license terms
 * in the LICENSE file found in the top-level directory of this distribution.
 * No part of rustlang-game, including this file, may be copied, modified, propagated,
 * or distributed except according to the terms contained in the LICENSE file.
 */

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time;

use block::Block;
use grid::Grid;
use point::direction;
use graphics::UI;

pub struct Game {
    grid:       Grid,
    block:      Block,
    score:      i32,
    gameover:   bool,
    timer:      time::Instant,
}

impl Game {

    pub fn new() -> Game {
        Game {
            grid:       Grid::new(),
            block:      Block::new(),
            score:      0,
            gameover:   false,
            timer:      time::Instant::now(),
        }
    }
    
    pub fn play(&mut self) {
        
        let mut ui = UI::new();

        self.grid.place(&self.block);

        'main: loop {
            
            for event in ui.event_pump.poll_iter() {

                if let Event::KeyDown {keycode, ..} = event {
                    
                    if let Some(keycode) = keycode {
                        
                        if keycode == Keycode::Up {
                            self.grid.rotate_block(&mut self.block);
                        }
                        else if keycode == Keycode::Down {
                            self.timer = time::Instant::now();
                            self.grid.move_block(&mut self.block, direction::DOWN);
                        }
                        else if keycode == Keycode::Left {
                            self.grid.move_block(&mut self.block, direction::LEFT);
                        }
                        else if keycode == Keycode::Right {
                            self.grid.move_block(&mut self.block, direction::RIGHT);
                        }
                    }
                }
                else if let Event::Quit {..} = event {
                    break 'main;
                }
            }

            if self.timer.elapsed() > time::Duration::new(0, 900000000) {
                if !self.grid.move_block(&mut self.block, direction::DOWN) {
                    self.block = Block::new();
                    self.grid.place(&self.block);
                }
                self.timer = time::Instant::now();
            }

            ui.clear();
            ui.draw_block(self.grid.get_cells());
            ui.render();
        }

        println!("Quitting");
    }
}
