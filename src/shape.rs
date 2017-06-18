pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub static L_SHAPE: [Point; 4] = [
    Point { x: 0, y: 4 },
    Point { x: 1, y: 4 },
    Point { x: 2, y: 4 },
    Point { x: 2, y: 5 },
];

pub static J_SHAPE: [Point; 4] = [
    Point { x: 0, y: 5 },
    Point { x: 1, y: 5 },
    Point { x: 2, y: 5 },
    Point { x: 2, y: 4 },
];

pub static O_SHAPE: [Point; 4] = [
    Point { x: 0, y: 4 },
    Point { x: 0, y: 5 },
    Point { x: 1, y: 4 },
    Point { x: 1, y: 5 },
];

pub static T_SHAPE: [Point; 4] = [
    Point { x: 0, y: 4 },
    Point { x: 0, y: 5 },
    Point { x: 0, y: 6 },
    Point { x: 1, y: 5 },
];

pub static S_SHAPE: [Point; 4] = [
    Point { x: 0, y: 6 },
    Point { x: 0, y: 5 },
    Point { x: 1, y: 5 },
    Point { x: 1, y: 4 },
];

pub static Z_SHAPE: [Point; 4] = [
    Point { x: 0, y: 4 },
    Point { x: 0, y: 5 },
    Point { x: 1, y: 5 },
    Point { x: 1, y: 6 },
];

pub static I_SHAPE: [Point; 4] = [
    Point { x: 0, y: 4 },
    Point { x: 1, y: 4 },
    Point { x: 2, y: 4 },
    Point { x: 3, y: 4 },
];

pub static NO_SHAPE: [Point; 0] = [];
