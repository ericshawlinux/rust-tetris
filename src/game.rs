use sdl2;
use sdl2::video;
use sdl2::event::Event;

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
                    key @ Event::KeyDown {..} => self.handle_key(key),
                    Event::Quit {..} => break 'main,
                    _ => {},
                }
            }
        }

        println!("Quitting");
    }

    fn handle_key(&mut self, key: Event) {
        
        if let Some(shape) = self.block.shape {
            
            self.block = GridBuilder::new()
                .with_color(self.block.color)
                .with_shape(shape)
                .with_rotation(shape::rotate(self.block.rotation))
                .finalize();

            self.block.print();
        }
    }
}
