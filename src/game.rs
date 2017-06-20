use sdl2;
use sdl2::video;
use sdl2::event::Event;

use grid::GridBuilder;
use grid::Grid;
use grid;
use shape;

pub struct Game {
    grid:       Grid,
    shape:      &'static shape::Shape,
    block:      Grid,
    offset:     shape::Point,
    rotation:   usize,
    score:      i32,
    gameover:   bool,
}

impl Game {

    pub fn new() -> Game {
        
        let block = GridBuilder::new()
            .with_color(grid::next_color())
            .with_shape(shape::next_shape());

        Game {
            grid:       GridBuilder::new().finalize(),
            shape:      block.get_shape(),
            offset:     block.get_offset(),
            rotation:   block.get_rotation(),
            block:      block.finalize(),
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

        let block = GridBuilder::new()
            .with_color(grid::Color::Red)
            .with_shape(self.shape)
            .with_rotation(shape::rotate(self.rotation));
        
        self.rotation = block.get_rotation();
        self.block = block.finalize();

        self.print_block();
    }

    fn print_block(&self) {

        for y in 0..self.block.len() {
            for x in 0..self.block[y].len() {
                match self.block[y][x] {
                    grid::Color::Empty => print!("_"),
                    _ => print!("X"),
                }
            }
            println!();
        }
    }
}