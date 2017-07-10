use rand;
use rand::Rng;
use sdl2::pixels::Color;
use point::Point;
use point::Plot;

pub struct Block {
    pub plot:   Plot,
    pub center: Point,
    pub color:  Color,
}

impl Block {

    pub fn new() -> Block {
        let n = rand::thread_rng().gen_range(1, 8);

        match n {
            1 => Block {
                plot: [Point { x: 5, y: 0 }, Point { x: 5, y: 1 }, Point { x: 5, y: 2 }, Point { x: 5, y: 3 }],
                center: Point { x: 5, y: 1 },
                color: Color::RGB(0, 255, 255),
            },
            2 => Block {
                plot: [Point { x: 5, y: 0 }, Point { x: 5, y: 1 }, Point { x: 5, y: 2 }, Point { x: 6, y: 2 }],
                center: Point { x: 5, y: 1 },
                color: Color::RGB(15, 100, 255),
            },
            3 => Block {
                plot: [Point { x: 6, y: 0 }, Point { x: 6, y: 1 }, Point { x: 5, y: 2 }, Point { x: 6, y: 2 }],
                center: Point { x: 6, y: 1 },
                color: Color::RGB(255, 165, 0),
            },
            4 => Block {
                plot: [Point { x: 4, y: 0 }, Point { x: 4, y: 1 }, Point { x: 5, y: 1 }, Point { x: 5, y: 0 }],
                center: Point { x: 5, y: 0 },
                color: Color::RGB(255, 255, 0),
            },
            5 => Block {
                plot: [Point { x: 6, y: 0 }, Point { x: 5, y: 0 }, Point { x: 5, y: 1 }, Point { x: 4, y: 1 }],
                center: Point { x: 5, y: 0 },
                color: Color::RGB(0, 255, 0),
            },
            6 => Block {
                plot: [Point { x: 5, y: 0 }, Point { x: 4, y: 1 }, Point { x: 5, y: 1 }, Point { x: 6, y: 1 }],
                center: Point { x: 5, y: 1 },
                color: Color::RGB(210, 0, 128),
            },
            7 => Block {
                plot: [Point { x: 4, y: 0 }, Point { x: 5, y: 0 }, Point { x: 5, y: 1 }, Point { x: 6, y: 1 }],
                center: Point { x: 5, y: 1 },
                color: Color::RGB(255, 0, 0),
            },
            _ => panic!("No such block")
        }
    }
}