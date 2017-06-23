use sdl2::event::Event;
use sdl2::event::EventType;
use sdl2::keyboard;
use sdl2::keyboard::Keycode;
use std::time;

use block::Block;
use grid::Grid;
use grid::Direction;
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
                        
                        if let Some(direction) = Direction::from_keycode(keycode) {
                            self.grid.move_block(&mut self.block, direction);
                        }
                    }
                }
                else if let Event::Quit {..} = event {
                    break 'main;
                }
            }

            ui.clear();
            ui.draw_block(self.grid.get_cells());
            ui.render();
        }

        println!("Quitting");
    }
}
