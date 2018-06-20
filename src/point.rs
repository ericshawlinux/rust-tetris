/* point.rs
 *  
 * Created by Eric Shaw Jr. on 2017-07-10
 * Copyright (c) 2017, 2018 Eric Shaw Jr.
 * 
 * This file is part of rustlang-game. It is subject to the license terms
 * in the LICENSE file found in the top-level directory of this distribution.
 * No part of rustlang-game, including this file, may be copied, modified, propagated,
 * or distributed except according to the terms contained in the LICENSE file.
 */

use std::ops::Sub;
use std::ops::Add;

// represents a point on the grid
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// represents a set of points
pub type Plot = [Point; 4];


pub mod direction {
    pub const LEFT:     super::Point = super::Point { x: -1, y:  0 };
    pub const RIGHT:    super::Point = super::Point { x:  1, y:  0 };
    pub const UP:       super::Point = super::Point { x:  0, y: -1 };
    pub const DOWN:     super::Point = super::Point { x:  0, y:  1 };
    pub const NONE:     super::Point = super::Point { x:  0, y:  0 };
}