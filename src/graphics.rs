use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use grid;
use color;

const CELL:     u32 = 50;
const CBORDER:  u32 = 4;
const SCORE:    u32 = 250;

pub struct UI {
    sdl_context:    sdl2::Sdl,
    pub event_pump: sdl2::EventPump,
    video:          sdl2::VideoSubsystem,
    canvas:         sdl2::render::WindowCanvas,
}

impl UI {

    pub fn new() -> UI {
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let video = sdl_context.video().unwrap();
        
        let window = video.window("New window", CELL * grid::GRID_WIDTH as u32 + SCORE + CBORDER, CELL * grid::GRID_HEIGHT as u32 + CBORDER)
            .position_centered()
            .build().unwrap();

        let mut canvas : sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas()
            .present_vsync()
            .build().unwrap();
        
        UI {
            sdl_context: sdl_context,
            event_pump: event_pump,
            video: video,
            canvas: canvas,
        }
    }

    pub fn clear(&mut self) -> &mut UI {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(0, 0, 150));
        self.canvas.fill_rect(Rect::new(CELL as i32 * grid::GRID_WIDTH as i32 + CBORDER as i32, 0, SCORE, CELL * grid::GRID_HEIGHT as u32 + CBORDER));

        self
    }

    pub fn draw_block(&mut self, cells: &grid::GridArray) -> &mut UI {
        
        for y in 0..cells.len() {
            for x in 0..cells[y].len() {
                if cells[y][x] != color::Color::Empty {
                    self.draw_block_segment(x, y, cells[y][x]);
                }
            }
        }
        
        self
    }

    fn draw_block_segment(&mut self, x: usize, y: usize, color: color::Color) -> &mut UI {
        let plot1 = CELL as i32 * x as i32 + CBORDER as i32;
        let plot2 = CELL as i32 * y as i32 + CBORDER as i32;
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.fill_rect(Rect::new(plot1, plot2, CELL, CELL));
        self.canvas.set_draw_color(color.to_rgb());
        self.canvas.fill_rect(Rect::new(plot1, plot2, CELL - CBORDER, CELL - CBORDER));
        self
    }

    pub fn render(&mut self) -> &mut UI {
        self.canvas.present();
        self
    }
}