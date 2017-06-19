extern crate rand;
pub mod grid;
pub mod shape;
use grid::GridBuilder;
use grid::Grid;

struct Game {
    grid:       Grid,
    block:      Grid,
    offset:     shape::Point,
    rotation:   usize,
    score:      i32,
    gameover:   bool,
}

impl Game {

    fn new() -> Game {
        
        let block = GridBuilder::new()
            .with_color(grid::next_color())
            .with_shape(shape::next_shape());

        Game {
            grid:       GridBuilder::new().finalize(),
            offset:     block.get_offset(),
            rotation:   shape::ROTATE_A,
            block:      block.finalize(),
            score:      0,
            gameover:   false,
        }
    }
    
    fn play(&mut self) {
    
        while !self.gameover {            
            
        }
    }
}


fn main() {
    let mut game = Game::new();
    game.play();
}
