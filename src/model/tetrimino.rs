use opengl_graphics::{GlGraphics};
use graphics::DrawState;
use graphics::context::Context;

use util::*;
use constants::*;
use model::shape::*;
/// The active tetrimino.
pub struct Tetrimino {
    pub x: i32,
    pub y: i32,
    pub shape: Shape,
    pub rotation: i32, // clockwise rotations.
}

impl Tetrimino {
    pub fn draw(&self, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        for tile in &self.shape.tiles(self.rotation) {
            let x = self.x + tile.0 + BOARD_OFFSET_X;
            let y = self.y + tile.1 + BOARD_OFFSET_Y;
            draw_square(x, y, self.shape.color(), c, draw_state, gl);
        }
    }

    pub fn tiles(&self) -> Vec<(i32, i32)> {
        self.shape
            .tiles(self.rotation)
            .iter()
            .map(|t| (t.0 + self.x, t.1 + self.y))
            .collect()
    }
    pub fn tiles_offset(&self, offset: (i32, i32)) -> Vec<(i32, i32)> {
        self.shape
            .tiles(self.rotation)
            .iter()
            .map(|t| (t.0 + self.x + offset.0, t.1 + self.y + offset.1))
            .collect()
    }

    pub fn rotate(&mut self) -> () {
        self.rotation = (self.rotation + 1) % 4;
    }

    // rotation
    pub fn tiles_rotated(&self) -> Vec<(i32, i32)> {
        self.shape
            .tiles((self.rotation + 1) % 4)
            .iter()
            .map(|t| (t.0 + self.x, t.1 + self.y))
            .collect()
    }
}
