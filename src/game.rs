use sdl2;
use sdl2::video;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use grid::GridBuilder;
use grid::Grid;
use grid;
use shape;

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
                            .with_color(grid::next_color())
                            .with_shape(shape::next_shape())
                            .finalize(),
            score:      0,
            gameover:   false,
        }
    }
    
    pub fn play(&mut self) {
        
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let video = sdl_context.video().unwrap();
        let window = video.window("New window", 400, 400)
            .position_centered()
            .build();

        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown {timestamp, window_id, keycode, ..} => self.handle_key(keycode),
                    Event::Quit {..} => break 'main,
                    _ => {},
                }
            }
        }

        println!("Quitting");
    }

    fn handle_key(&mut self, key: Option<Keycode>) {
        
        if let (Some(shape), Some(key)) = (self.block.shape, key) {
            
            let mut block = GridBuilder::new();
            
            block.with_color(self.block.color)
                .with_shape(shape)
                .with_rotation(self.block.rotation);
            
            if key == Keycode::S {
                block.with_shape(shape::next_shape());
            }
            else if key == Keycode::C {
                block.with_color(grid::next_color());
            }
            else if key == Keycode::Up {
                block.with_rotation(shape::rotate(self.block.rotation));
            }
            
            self.block = block.finalize();
            self.block.print();
        }
    }
}
