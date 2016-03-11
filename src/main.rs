extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

/// Halvors
/// YEAH

const WINDOW_TITLE: &'static str = "FUTRIS";
const TILE_SIZE: i32 = 32;
const MAP_WIDTH: i32 = 10;
const MAP_HEIGHT: i32 = 30;

/// The board itself.

trait Action {
    fn direction(tetrimino: &mut Tetrimino) -> (i32, i32) {
        (0,0)
    }
    fn rotation(tetrimino: &mut Tetrimino) -> (i32) {
        0
    }
}

struct MoveLeft;
impl Action for MoveLeft{
    fn direction(tetrimino: &mut Tetrimino)-> (i32, i32) {
        (-1, 0)
    }
}

struct MoveRight;
impl Action for MoveRight {
    fn direction(tetrimino: &mut Tetrimino) -> (i32, i32) {
        (1, 0)
    }
}

struct MoveDown;
impl Action for MoveDown {
    fn direction(tetrimino: &mut Tetrimino) -> (i32, i32) {
        (0, 1)
    }
}

struct RotateRight;
impl Action for RotateRight {
    fn rotation(tetrimino: &mut Tetrimino) -> i32 {
        1
    }
}

struct RotateLeft;
impl Action for RotateLeft {
    fn rotation(tetrimino: &mut Tetrimino) -> i32 {
        -1
    }
}

struct TetrisBoard {
    dead_tiles: Vec<Box<DeadTile>>,
    active_tetrimino: Tetrimino,
}

impl TetrisBoard {
    fn move_tetrimino(&mut self) -> () {
        println!("MOVE!")
    }
}

/// Tetriminos that have landed.
struct DeadTile {
    x: i32,
    y: i32,
    // this is included for the color.
    tetrimino_shape: TetriminoShape,
}

/// The active tetrimino.
struct Tetrimino {
    x: i32,
    y: i32,
    tetrimino_shape: Box<TetriminoShape>,
    rotation: i32, // clockwise rotations.
}

impl Tetrimino {
    fn draw_tetrimino(&self) -> () {
        println!("hey")
    }
}

trait TetriminoShape {
    fn name(&self) -> &str;
    fn color(&self) -> [f32; 4];

    // tiles: an array of length 4 with relative positions to origin (top left of spawn)
    // 0.0 1.0
    // 0.1 1.1
    fn tiles(&self) -> [(i32, i32); 4];
}

struct I;
impl TetriminoShape for I {
    fn name(&self) -> &str {
        "I"
    }

    fn color(&self) -> [f32; 4] {
        // red
        [0.95, 0.26, 0.21, 1.0]
    }

    fn tiles(&self) -> [(i32, i32); 4] {
        [(0, 0), (1, 0), (2, 0), (3, 0)]
    }
}


struct O;
impl TetriminoShape for O {
    fn name(&self) -> &str {
        "O"
    }

    fn color(&self) -> [f32; 4] {
        // blue
        [0.13, 0.59, 0.95, 1.0]
    }

    fn tiles(&self) -> [(i32, i32); 4] {
        [(0, 0), (1, 0), (0, 1), (1, 1)]
    }
}

struct T;
impl TetriminoShape for T {
    fn name(&self) -> &str {
        "T"
    }

    fn color(&self) -> [f32; 4] {
        // lime
        [0.80, 0.86, 0.22, 1.0]
    }

    fn tiles(&self) -> [(i32, i32); 4] {
        [(0, 0), (1, 0), (2, 0), (1, 1)]
    }
}

struct S;
impl TetriminoShape for S {
    fn name(&self) -> &str {
        "S"
    }

    fn color(&self) -> [f32; 4] {
        // cyan
        [0.0, 0.73, 0.83, 1.0]
    }

    fn tiles(&self) -> [(i32, i32); 4] {
        [(0, 0), (1, 0), (-1, 1), (0, 1)]
    }
}

struct Z;
impl TetriminoShape for Z {
    fn name(&self) -> &str {
        "Z"
    }

    fn color(&self) -> [f32; 4] {
        // orange
        [1.0, 0.60, 0.0, 1.0]
    }

    fn tiles(&self) -> [(i32, i32); 4] {
        [(0, 0), (0, 1), (1, 1), (2, 1)]
    }
}

struct J;
impl TetriminoShape for J {
    fn name(&self) -> &str {
        "J"
    }

    fn color(&self) -> [f32; 4] {
        // yellow
        [1.0, 0.92, 0.23, 1.0]
    }

    fn tiles(&self) -> [(i32, i32); 4] {
        [(0, 0), (1, 0), (2, 0), (2, 1)]
    }
}

struct L;
impl TetriminoShape for L {
    fn name(&self) -> &str {
        "L"
    }

    fn color(&self) -> [f32; 4] {
        // magenta (really pink, should be magenta though)
        [0.91, 0.11, 0.39, 1.0]
    }

    fn tiles(&self) -> [(i32, i32); 4] {
        [(0, 0), (1, 0), (2, 0), (0, 1)]
    }
}



/// END Halvors
/// YEAH

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64, // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform
                             .trans(x, y)
                             .rot_rad(rotation)
                             .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(WINDOW_TITLE, [200, 200])
                                 .opengl(opengl)
                                 .exit_on_esc(true)
                                 .build()
                                 .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
