use rand;
use rand::Rng;
use sdl2::pixels::Color;
use point::Point;
use point::Plot;
use grid::GridArray;
use grid::GRID_HEIGHT;
use grid::GRID_WIDTH;

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

    // returns a new plot translated by 'change'
    pub fn translate_plot(plot: &Plot, change: Point) -> Plot {

        let mut result: Plot = *plot;

        for p in 0..plot.len() {
            result[p] = plot[p] + change;
        }

        result
    }

    pub fn validate_plot(&self, cells: &mut GridArray, new_plot: &Plot) -> bool {
        'validation:
        for point in new_plot {

            // collision with self is ok, skip
            for current_point in &self.plot {
                if (point.x, point.y) == (current_point.x, current_point.y) {
                    continue 'validation;
                }
            }
            // check grid boundaries
            if point.x < 0 || point.y < 0 {
                println!("Out of bounds");
                return false;
            }
            if point.x >= GRID_WIDTH as i32 || point.y >= GRID_HEIGHT as i32 {
                println!("Out of bounds");
                return false;
            }
            // check for collision
            if cells[point.y as usize][point.x as usize] != Color::RGB(0, 0, 0) {
                println!("Collision");
                return false;
            }
        }

        true
    }

    pub fn finalize_plot(&mut self, cells: &mut GridArray, new_plot: Plot, change: Point) {

        // erase old block
        for point in &self.plot {
            cells[point.y as usize][point.x as usize] = Color::RGB(0, 0, 0);
        }

        // update block plot
        self.plot = new_plot;
        self.center = self.center + change;

        // update grid
        for point in &self.plot {
            cells[point.y as usize][point.x as usize] = self.color;
        }
    }
}
