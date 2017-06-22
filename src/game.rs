use sdl2::event::Event;
use sdl2::keyboard;
use sdl2::keyboard::Keycode;
use std::time;

use grid;
use grid::Grid;
use grid::GridBuilder;
use color;
use shape;
use shape::Point;
use graphics::UI;

pub struct Game {
    grid:       Grid, // past blocks that have been set
    block:      Grid, // current block
    score:      i32,
    gameover:   bool,
    timer:      time::Instant,
}

impl Game {

    pub fn new() -> Game {
        Game {
            grid:       GridBuilder::new().finalize(),
            block:      GridBuilder::new()
                            .with_shape(shape::next_shape())
                            .finalize(),
            score:      0,
            gameover:   false,
            timer:      time::Instant::now(),
        }
    }
    
    pub fn play(&mut self) {
        
        let mut ui = UI::new();

        'main: loop {
            
            for event in ui.event_pump.poll_iter() {
                match event {
                    Event::KeyDown {keycode, keymod, ..} => {
                        if keymod.contains(keyboard::LSHIFTMOD) {
                            self.handle_key_debug(keycode);
                        }
                        self.handle_key(keycode);
                    },
                    Event::Quit {..} => break 'main,
                    _ => {},
                }
            }

            self.fall();

            ui.clear();
            ui.draw_block(self.grid.get_cells());
            ui.draw_block(self.block.get_cells());
            ui.render();
        }

        println!("Quitting");
    }

    fn fall(&mut self) {

        if self.timer.elapsed() < time::Duration::new(1, 0) {
            return;
        }

        let mut fallen_block = GridBuilder::new();
        
        self.block = fallen_block
            .with_shape((self.block.color, self.block.shape.unwrap()))
            .with_offset(self.block.offset)
            .with_rotation(self.block.rotation)
            .move_down().finalize();

        self.timer = time::Instant::now();
    }

    fn handle_key(&mut self, key: Option<Keycode>) {

        if let (Some(shape), Some(key)) = (self.block.shape, key) {
            
            let mut block = GridBuilder::new();
            
            block.with_shape((self.block.color, shape))
                .with_offset(self.block.offset)
                .with_rotation(self.block.rotation);  
            
            if key == Keycode::Up {
                block.with_rotation(shape::rotate(self.block.rotation));
            }
            else if key == Keycode::Down {
                self.timer = time::Instant::now();
                block.move_down();
            }
            else if key == Keycode::Left {
                block.move_left();
            }
            else if key == Keycode::Right {
                block.move_right();
            }

            self.block = block.finalize();
        }
    }

    #[allow(dead_code)]
    fn handle_key_debug(&mut self, key: Option<Keycode>) {
        
        if let (Some(shape), Some(key)) = (self.block.shape, key) {
            
            let mut block = GridBuilder::new();
            
            block.with_shape((self.block.color, shape))
                .with_offset(self.block.offset)
                .with_rotation(self.block.rotation);            
            
            if key == Keycode::S {
                block.with_shape(shape::next_shape());
            }
            else if key == Keycode::R {
                block.with_rotation(shape::rotate(self.block.rotation));
            }
            else if key == Keycode::N {
                block.with_shape(shape::next_shape());
                block.with_offset(Point { x: 4, y: -1 });
            }
            else if key == Keycode::Up {
                block.move_up();
            }
            else if key == Keycode::Down {
                block.move_down();
            }
            else if key == Keycode::Left {
                block.move_left();
            }
            else if key == Keycode::Right {
                block.move_right();
            }

            self.block = block.finalize();
            
            if key == Keycode::Space {
                self.grid.place(self.block.get_cells());
                self.grid.print();
            }
        }
    }
}
