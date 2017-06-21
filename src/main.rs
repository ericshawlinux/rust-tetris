extern crate rand;
extern crate sdl2;

pub mod grid;
pub mod shape;
pub mod game;
pub mod color;
pub mod graphics;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.play();
}
