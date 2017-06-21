use rand;
use rand::Rng;

// shape plots, including rotations
pub type Shape = [[Point; 4]; 4];

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn rotate(current: usize) -> usize {
    if current >= MAX_ROTATE {
        ROTATE_A
    }
    else {
        current + 1
    }
}

pub const ROTATE_A:     usize = 0;
pub const ROTATE_B:     usize = 1;
pub const ROTATE_C:     usize = 2;
pub const ROTATE_D:     usize = 3;
pub const MAX_ROTATE:   usize = 3;

pub fn next_shape() -> &'static Shape {
    let n = rand::thread_rng().gen_range(1, 8);
    
    match n {
        1 => &I_SHAPES,
        2 => &J_SHAPES,
        3 => &L_SHAPES,
        4 => &O_SHAPES,
        5 => &S_SHAPES,
        6 => &T_SHAPES,
        7 => &Z_SHAPES,
        _ => panic!("no such shape"),
    }
}

pub static L_SHAPES: Shape = [
    // no rotation
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 0, y: 2 }, Point { x: 1, y: 2 }],
    // 1 rotation
    [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 0, y: 1 }],
    // 2 rotations
    [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }],
    // 3 rotations
    [Point { x: 2, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
];

pub static J_SHAPES: Shape = [
    [Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 0, y: 2 }, Point { x: 1, y: 2 }],
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
    [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 0, y: 1 }, Point { x: 0, y: 2 }],
    [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 2, y: 1 }],
];

pub static S_SHAPES: Shape = [
    [Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }],
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }],
    [Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }],
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }],
];

pub static Z_SHAPES: Shape = [
    [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
    [Point { x: 1, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 0, y: 2 }],
    [Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
    [Point { x: 1, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 0, y: 2 }],
];

pub static T_SHAPES: Shape = [
    [Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }, Point { x: 1, y: 2 }],
    [Point { x: 1, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }],
    [Point { x: 1, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }],
    [Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }, Point { x: 1, y: 2 }],
];

pub static I_SHAPES: Shape = [
    [Point { x: 1, y: 0 }, Point { x: 1, y: 1 }, Point { x: 1, y: 2 }, Point { x: 1, y: 3 }],
    [Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 }, Point { x: 3, y: 1 }],
    [Point { x: 2, y: 0 }, Point { x: 2, y: 1 }, Point { x: 2, y: 2 }, Point { x: 2, y: 3 }],
    [Point { x: 0, y: 2 }, Point { x: 1, y: 2 }, Point { x: 2, y: 2 }, Point { x: 3, y: 2 }],
];

pub static O_SHAPES: Shape = [
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 0 }],
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 0 }],
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 0 }],
    [Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 1, y: 0 }],
];

