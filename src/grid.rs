use std::cmp;

use shape;
use shape::Point;
use shape::Shape;
use color::Color;

pub const GRID_HEIGHT:      usize = 20;
pub const GRID_WIDTH:       usize = 10;

pub struct Grid {
    pub shape:      Option<&'static Shape>,
    pub offset:     Point,
    pub rotation:   usize,
    pub color:      Color,
    cells:          GridArray,
}

impl Grid {
    pub fn print(&self) {

        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                match self.cells[y][x] {
                    Color::Empty => print!("_"),
                    _ => print!("X"),
                }
            }
            println!();
        }
    }
    
    pub fn get_cells(&self) -> &GridArray {
    
        &self.cells
    }

    pub fn place(&mut self, grid: &GridArray) -> &mut Grid {
        
        println!("Placing");
        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                if grid[y][x] != Color::Empty {
                    self.cells[y][x] = grid[y][x];
                }
            }
        }
        self
    }
}

pub type GridArray = [[Color; GRID_WIDTH]; GRID_HEIGHT];

pub struct GridBuilder {
    shape:      Option<&'static Shape>,
    offset:     Point,
    rotation:   usize,
    color:      Color,
    cells:      GridArray,
}

impl GridBuilder {

    pub fn new() -> GridBuilder {
    
        GridBuilder {
            shape:      None,
            offset:     Point { x: 4, y: -1 }, // centered at top
            rotation:   shape::ROTATE_A,
            color:      Color::Empty,
            cells:      [[Color::Empty; GRID_WIDTH]; GRID_HEIGHT],
        }
    }
    
    pub fn with_shape(&mut self, shape: (Color, &'static Shape)) -> &mut GridBuilder {
        
        let (color, shape) = (shape.0, shape.1);

        self.color = color;
        self.shape = Some(shape);
        self
    }

    pub fn with_offset(&mut self, offset: Point) -> &mut GridBuilder {
        
        self.offset = offset;
        self
    }
    
    pub fn move_up(&mut self) -> &mut GridBuilder {
    
        self.offset.y -= 1;
        self
    }
    
    pub fn move_down(&mut self) -> &mut GridBuilder {
    
        self.offset.y += 1;
        self
    }
    
    pub fn move_left(&mut self) -> &mut GridBuilder {
    
        self.offset.x -= 1;
        self
    }
    
    pub fn move_right(&mut self) -> &mut GridBuilder {
    
        self.offset.x += 1;
        self
    }

    pub fn with_rotation(&mut self, rotation: usize) -> &mut GridBuilder {

        self.rotation = rotation;
        self
    }
    
    pub fn finalize (&mut self) -> Grid {
        
        let mut grid = Grid {
            shape:      self.shape,
            offset:     self.offset,
            rotation:   self.rotation,
            color:      self.color,
            cells:      self.cells,
        };
        
        if let Some(shape) = self.shape {
            
            let shape = shape[self.rotation];
            
            // check array boundaries are not crossed (4 points)
            for point_idx in 0..shape.len() {

                let point = &shape[point_idx];

                if point.x + self.offset.x >= GRID_WIDTH as i32 {
                    self.offset.x -= 1;
                    grid.offset.x -= 1;
                }
                if point.x + self.offset.x < 0 {
                    self.offset.x += 1;
                    grid.offset.x += 1;
                }
                if point.y + self.offset.y >= GRID_HEIGHT as i32 {
                    self.offset.y -= 1;
                    grid.offset.y -= 1;
                }
                if point.y + self.offset.y < 0 {
                    self.offset.y += 1;
                    grid.offset.y += 1;
                }
            }
            
            // draw the grid with shape (same 4 points again)
            for point_idx in 0..shape.len() {

                let point = &shape[point_idx];
                let x = (point.x + self.offset.x) as usize;
                let y = (point.y + self.offset.y) as usize;

                grid.cells[y][x] = self.color;
            }
        }
        
        grid
    }
}
