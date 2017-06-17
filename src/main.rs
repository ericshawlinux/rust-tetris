pub mod grid;
pub mod shape;
use grid::*;

struct Game {
    grid:       Grid,
    block:      Grid,
    score:      i32,
    gameover:   bool,
}

impl Game {

    fn new() -> Game {
        
        Game {
            grid:   GridBuilder::new().finalize(),
            block:  GridBuilder::new()
                        .with_color(Color::Red)
                        .with_shape(&shape::L_SHAPE)
                        .finalize(),
            score:  0,
            gameover: false,
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
