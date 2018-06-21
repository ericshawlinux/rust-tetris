/* graphics.rs
 *  
 * Created by Eric Shaw Jr. on 2017-06-21
 * Copyright (c) 2017, 2018 Eric Shaw Jr.
 * 
 * This file is part of rustlang-game. It is subject to the license terms
 * in the LICENSE file found in the top-level directory of this distribution.
 * No part of rustlang-game, including this file, may be copied, modified, propagated,
 * or distributed except according to the terms contained in the LICENSE file.
 */

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use grid;

const CELL:     u32 = 25;
const CBORDER:  u32 = 2;
const SCORE:    u32 = 150;

pub struct UI {
    pub sdl_context:    sdl2::Sdl,
    video:              sdl2::VideoSubsystem,
    canvas:             sdl2::render::WindowCanvas,
}

impl UI {

    pub fn new() -> UI {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        
        let window = video.window("Game", CELL * grid::GRID_WIDTH as u32 + SCORE + CBORDER, CELL * grid::GRID_HEIGHT as u32 + CBORDER)
            .resizable()
            .position_centered()
            .build().unwrap();

        let canvas : sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas()
            .present_vsync()
            .build().unwrap();
        
        UI {
            sdl_context: sdl_context,
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
                if cells[y][x] != Color::RGB(0, 0, 0) {
                    self.draw_block_segment(x, y, cells[y][x]);
                }
            }
        }
        
        self
    }

    fn draw_block_segment(&mut self, x: usize, y: usize, color: Color) -> &mut UI {
        let plot1 = CELL as i32 * x as i32 + CBORDER as i32;
        let plot2 = CELL as i32 * y as i32 + CBORDER as i32;
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.fill_rect(Rect::new(plot1, plot2, CELL, CELL));
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(plot1, plot2, CELL - CBORDER, CELL - CBORDER));
        self
    }

    pub fn render(&mut self) -> &mut UI {
        self.canvas.present();
        self
    }

    pub fn resize(&mut self, height: i32, width: i32) -> &mut UI {
        
        self
    }
}