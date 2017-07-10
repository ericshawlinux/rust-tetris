extern crate rand;
extern crate sdl2;

pub mod block;
pub mod game;
pub mod graphics;
pub mod grid;
pub mod point;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.play();
}
