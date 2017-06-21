use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
}

impl Game {

    pub fn new() -> Game {
        Game {
            grid:       GridBuilder::new().finalize(),
            block:      GridBuilder::new()
                            .with_color(color::next_color())
                            .with_shape(shape::next_shape())
                            .finalize(),
            score:      0,
            gameover:   false,
        }
    }
    
    pub fn play(&mut self) {
        
        let mut ui = UI::new();
        
        'main: loop {
            
            for event in ui.event_pump.poll_iter() {
                match event {
                    Event::KeyDown {keycode, ..} => self.handle_key(keycode),
                    Event::Quit {..} => break 'main,
                    _ => {},
                }
            }

            ui.clear();
            ui.draw_block(self.grid.get_cells());
            ui.draw_block(self.block.get_cells());
            ui.render();
        }

        println!("Quitting");
    }

    fn handle_key(&mut self, key: Option<Keycode>) {
        
        if let (Some(shape), Some(key)) = (self.block.shape, key) {
            
            let mut block = GridBuilder::new();
            
            block.with_color(self.block.color)
                .with_shape(shape)
                .with_offset(self.block.offset)
                .with_rotation(self.block.rotation);            
            
            if key == Keycode::S {
                block.with_shape(shape::next_shape());
            }
            else if key == Keycode::C {
                block.with_color(color::next_color());
            }
            else if key == Keycode::R {
                block.with_rotation(shape::rotate(self.block.rotation));
            }
            else if key == Keycode::N {
                block.with_shape(shape::next_shape());
                block.with_offset(Point { x: 4, y: -1 });
                block.with_color(color::next_color());
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
