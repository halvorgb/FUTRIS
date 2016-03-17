extern crate rand;

use constants::*;
use self::rand::{Rng, Rand};
use self::rand::distributions::{IndependentSample, Range};

pub enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Shape {
    pub fn color(&self) -> [f32; 4] {
        match *self {
            Shape::I => [0.00, 0.73, 0.83, 1.0], // cyan
            Shape::O => [1.00, 0.92, 0.23, 1.0], // yellow
            Shape::T => [0.61, 0.15, 0.69, 1.0], // purple
            Shape::S => [0.55, 0.76, 0.29, 1.0], // light green
            Shape::Z => [0.95, 0.26, 0.21, 1.0], // red
            Shape::J => [0.13, 0.59, 0.95, 1.0], // blue
            Shape::L => [1.00, 0.60, 0.00, 1.0], // orange
        }
    }

    pub fn tiles(&self, rotation: i32) -> Vec<(i32, i32)> {
        match *self {
            Shape::I => {
                match rotation {
                    0 => vec![(0, 1), (1, 1), (2, 1), (3, 1)],
                    1 => vec![(2, 0), (2, 1), (2, 2), (2, 3)],
                    2 => vec![(0, 2), (1, 2), (2, 2), (3, 2)],
                    _ => vec![(1, 0), (1, 1), (1, 2), (1, 3)],
                }
            }
            Shape::O => {
                match rotation {
                    _ => vec![(1, 0), (2, 0), (1, 1), (2, 1)],
                }
            }

            Shape::T => {
                match rotation {
                    0 => vec![(1, 0), (0, 1), (1, 1), (2, 1)],
                    1 => vec![(1, 0), (1, 1), (1, 2), (2, 1)],
                    2 => vec![(0, 1), (1, 1), (2, 1), (1, 2)],
                    _ => vec![(1, 0), (1, 1), (1, 2), (0, 1)],
                }
            }

            Shape::S => {
                match rotation {
                    0 => vec![(1, 0), (2, 0), (0, 1), (1, 1)],
                    1 => vec![(1, 0), (1, 1), (2, 1), (2, 2)],
                    2 => vec![(1, 1), (2, 1), (0, 2), (1, 2)],
                    _ => vec![(0, 0), (0, 1), (1, 1), (1, 2)],
                }
            }

            Shape::Z => {
                match rotation {
                    0 => vec![(0, 0), (1, 0), (1, 1), (2, 1)],
                    1 => vec![(2, 0), (2, 1), (1, 1), (1, 2)],
                    2 => vec![(0, 1), (1, 1), (1, 2), (2, 2)],
                    _ => vec![(1, 0), (1, 1), (0, 1), (0, 2)],
                }
            }

            Shape::J => {
                match rotation {
                    0 => vec![(0, 0), (0, 1), (1, 1), (2, 1)],
                    1 => vec![(1, 0), (2, 0), (1, 1), (1, 2)],
                    2 => vec![(0, 1), (1, 1), (2, 1), (2, 2)],
                    _ => vec![(2, 0), (2, 1), (2, 2), (1, 2)],
                }
            }

            Shape::L => {
                match rotation {
                    0 => vec![(2, 0), (0, 1), (1, 1), (2, 1)],
                    1 => vec![(1, 0), (1, 1), (1, 2), (2, 2)],
                    2 => vec![(0, 1), (1, 1), (2, 1), (0, 2)],
                    _ => vec![(0, 0), (1, 0), (1, 1), (1, 2)],
                }
            }
        }
    }

    pub fn origin(&self) -> i32 {
        match *self {
            Shape::I => BOARD_WIDTH / 2 - 2,
            Shape::O => BOARD_WIDTH / 2 - 2,
            Shape::T => BOARD_WIDTH / 2 - 2,
            Shape::S => BOARD_WIDTH / 2 - 1,
            Shape::Z => BOARD_WIDTH / 2 - 2,
            Shape::J => BOARD_WIDTH / 2 - 2,
            Shape::L => BOARD_WIDTH / 2 - 2,
        }
    }
}

impl Rand for Shape {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let between: Range<i32> = Range::new(0, 7);
        match between.ind_sample(rng) {
            0 => Shape::I,
            1 => Shape::O,
            2 => Shape::T,
            3 => Shape::S,
            4 => Shape::Z,
            5 => Shape::J,
            6 => Shape::L,
            _ => panic!("what"),
        }
    }
}
