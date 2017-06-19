use rand;
use rand::Rng;

// shape plots, including rotations
pub type Shape = [[Point; 4]; 4];

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub const ROTATE_A: usize = 0;
pub const ROTATE_B: usize = 1;
pub const ROTATE_C: usize = 2;
pub const ROTATE_D: usize = 3;

pub fn next_shape() -> &'static Shape {
    let n = rand::thread_rng().gen_range(1, 8);
    
    match n {
        1 => &L_SHAPES,
        _ => &O_SHAPES,
    }
}

pub static L_SHAPES: Shape = [
    // no rotation
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 0, y: 2 }, Point { x: 1, y: 2 }],
    // 1 rotation
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }],
    // 2 rotations
    [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }],
    // 3 rotations
    [Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }, Point { x: 2, y: 0 }],
];

pub static O_SHAPES: Shape = [
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 0 }],
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 0 }],
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 0 }],
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 0 }],
];


