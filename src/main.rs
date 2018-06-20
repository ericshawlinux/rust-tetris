/* main.rs
 *  
 * Created by Eric Shaw Jr. on 2017-06-18
 * Copyright (c) 2017, 2018 Eric Shaw Jr.
 * 
 * This file is part of rustlang-game. It is subject to the license terms
 * in the LICENSE file found in the top-level directory of this distribution.
 * No part of rustlang-game, including this file, may be copied, modified, propagated,
 * or distributed except according to the terms contained in the LICENSE file.
 */

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
