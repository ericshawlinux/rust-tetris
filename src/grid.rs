use shape;
use shape::Point;
use rand;
use rand::Rng;

const GRID_HEIGHT:  usize = 22;
const GRID_WIDTH:   usize = 10;

pub type Grid = [[Color; GRID_WIDTH]; GRID_HEIGHT];

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
    Empty,
}

pub fn next_color() -> Color {
    let n = rand::thread_rng().gen_range(1, 8);

    match n {
        1 => Color::Red,
        2 => Color::Orange,
        3 => Color::Yellow,
        4 => Color::Green,
        5 => Color::Blue,
        6 => Color::Indigo,
        _ => Color::Violet,
    }
}


pub struct GridBuilder {
    shape: &'static [Point],
    color: Color,
    grid: Grid,
}

impl GridBuilder {

    pub fn new() -> GridBuilder {
    
        GridBuilder {
            shape:  &shape::NO_SHAPE,
            color:  Color::Empty,
            grid:   [[Color::Empty; GRID_WIDTH]; GRID_HEIGHT],
        }
    }
    
    pub fn with_color(mut self, color: Color) -> GridBuilder {
    
        self.color = color;
        self
    }
    
    pub fn with_shape(mut self, shape: &'static [Point]) -> GridBuilder {
    
        self.shape = shape;
        self
    }
    
    pub fn finalize (mut self) -> Grid {
        
        let shape = self.shape;
        
        for point_idx in 0..shape.len() {

            let point = &shape[point_idx];
            
            self.grid[point.x][point.y] = self.color;
        }
        
        self.grid
    }
    
}

