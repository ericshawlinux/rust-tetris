use shape;
use shape::Point;
use shape::Shape;
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
    shape:      Option<&'static Shape>,
    offset:     Point,
    rotation:   usize,
    color:      Color,
    grid:       Grid,
}

impl GridBuilder {

    pub fn new() -> GridBuilder {
    
        GridBuilder {
            shape:      None,
            offset:     Point { x: 3, y: 0 }, // centered at top
            rotation:   shape::ROTATE_A,
            color:      Color::Empty,
            grid:       [[Color::Empty; GRID_WIDTH]; GRID_HEIGHT],
        }
    }
    
    pub fn with_color(mut self, color: Color) -> GridBuilder {
    
        self.color = color;
        self
    }
    
    pub fn with_shape(mut self, shape: &'static Shape) -> GridBuilder {
    
        self.shape = Some(shape);
        self
    }

    pub fn get_shape(&self) -> &'static Shape {
        
        match self.shape {
            Some(shape) => shape,
            None => panic!("Can't get shape!"),
        }
    }

    pub fn with_offset(mut self, offset: Point) -> GridBuilder {

        self.offset = offset;
        self
    }

    pub fn get_offset(&self) -> Point {
        
        self.offset
    }

    pub fn with_rotation(mut self, rotation: usize) -> GridBuilder {

        self.rotation = rotation;
        self
    }

    pub fn get_rotation(&self) -> usize {
        
        self.rotation
    }
    
    pub fn finalize (mut self) -> Grid {
        
        if let Some(shape) = self.shape {
            
            let shape = shape[self.rotation];

            // check boundaries are not crossed (4 points)
            for point_idx in 0..shape.len() {

                let point = &shape[point_idx];

                // todo: make this actually work. gets messed up if get_offset previously gave a bad offset
                if point.x + self.offset.x >= GRID_WIDTH {
                    self.offset.x -= 1;
                }
                if point.y + self.offset.y >= GRID_HEIGHT {
                    self.offset.y -= 1;
                }
            }
            
            // draw the grid with shape (same 4 points again)
            for point_idx in 0..shape.len() {

                let point = &shape[point_idx];
                
                self.grid[point.x + self.offset.x][point.y + self.offset.y] = self.color;
            }
        }
        
        self.grid
    }
    
}

