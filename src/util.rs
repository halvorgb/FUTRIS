use opengl_graphics::{GlGraphics};
use graphics::DrawState;
use graphics::context::Context;
use constants::*;

pub fn draw_square(x: i32,
               y: i32,
               color: [f32; 4],
               c: Context,
               draw_state: &DrawState,
               gl: &mut GlGraphics)
               -> () {
    use graphics::*;
    let square = rectangle::square(0.0, 0.0, TILE_SIZE as f64);
    let border = rectangle::Border {
        color: [0.0, 0.0, 0.0, 1.0],
        radius: 1.0,
    };

    let rectangle = rectangle::Rectangle {
        color: color,
        shape: rectangle::Shape::Bevel(4.0),
        border: Some(border),
    };

    let transform = c.transform
        .trans((x as f64) * TILE_SIZE, (y as f64) * TILE_SIZE);

    rectangle.draw(square, draw_state, transform, gl);
}
