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
