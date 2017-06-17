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

pub static NO_SHAPE: [Point; 0] = [];
