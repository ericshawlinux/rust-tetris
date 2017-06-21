use sdl2;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::*;

use grid;
use grid::Grid;
use grid::GridBuilder;
use color;
use shape;

const CELL:     u32 = 50;
const SCORE:    u32 = 250;

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
        
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let video = sdl_context.video().unwrap();
        
        let window = video.window("New window", CELL * grid::GRID_WIDTH_U32 + SCORE, CELL * grid::GRID_HEIGHT_U32)
            .position_centered()
            .build().unwrap();

        let mut canvas : Canvas<Window> = window.into_canvas()
            .present_vsync()
            .build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 210, 0));
        canvas.fill_rect(Rect::new(0, 0, CELL * grid::GRID_WIDTH_U32, CELL * grid::GRID_HEIGHT_U32));
        
        'main: loop {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
        
            canvas.set_draw_color(Color::RGB(255, 210, 0));
            canvas.fill_rect(Rect::new(0, 0, CELL * grid::GRID_WIDTH_U32, CELL * grid::GRID_HEIGHT_U32));
            
            canvas.set_draw_color(Color::RGB(255, 50, 50));
            
            {
                let cells = self.block.get_cells();
                
                for y in 0..cells.len() {
                    for x in 0..cells[y].len() {
                        if cells[y][x] != color::Color::Empty {
                            let plot1 = CELL as i32 * x as i32;
                            let plot2 = CELL as i32 * y as i32;
                            let plot3 = (CELL as i32 * x as i32) + CELL as i32;
                            let plot4 = (CELL as i32 * y as i32) + CELL as i32;
                            canvas.fill_rect(Rect::new(plot1, plot2, plot3 as u32, plot4 as u32));
                        }
                    }
                }
            }
            
            canvas.present();
            
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown {keycode, ..} => self.handle_key(keycode),
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
            self.block.print();
        }
    }
}
