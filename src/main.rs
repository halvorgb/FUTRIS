extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::{Rand, Rng, SeedableRng, StdRng};
use rand::distributions::{IndependentSample, Range};

const WINDOW_TITLE: &'static str = "FUTRIS";
const TILE_SIZE: i32 = 32;
const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 30;
const INITIAL_MS_PER_DROP: i32 = 10;

pub struct Tetris {
    gl: GlGraphics, // OpenGL drawing backend.
    background_color: [f32; 4],
    board: Board, // the game state
}
impl Tetris {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [0.95, 0.26, 0.21, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = 0.0;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        let bgc = self.background_color;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(bgc, gl);

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
        //self.rotation += 2.0 * args.dt;
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


    let seed: &[_] = &[1,2,3,4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut board = Board::initial_board(rng);
    // Create a new game and run it.
    let mut tetris = Tetris {
        gl: GlGraphics::new(opengl),
        background_color: [0.1, 0.08, 0.12, 1.0],
        board: board,
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            tetris.render(&r);
        }

        if let Some(u) = e.update_args() {
            tetris.update(&u);
        }
    }
}

enum Action {
    MOVL,
    MOVR,
    DROP,
    ROTR,
    ROTL,
}

impl Action {
    fn transpose(&self) -> (i32, i32) {
        match *self {
            Action::MOVL => (-1, 0),
            Action::MOVR => (1, 0),
            Action::DROP => (0, 1),
            _ => (0, 0),
        }
    }

    fn rotate(&self) -> (i32) {
        match *self {
            Action::ROTR => 1,
            Action::ROTL => -1,
            _ => 0,
        }
    }

}

/// The board itself.
struct Board {
    dead_tiles: Vec<Box<DeadTile>>,
    tetrimino: Tetrimino,
    ms_per_drop: i32,
}

impl Board {
    fn move_tetrimino(&mut self) -> () {
        println!("MOVE!")
    }

    fn initial_board(rng: StdRng) -> Board {
        Board {
            dead_tiles: Vec::new(),
            tetrimino: Board::random_tetrimino(rng),
            ms_per_drop: INITIAL_MS_PER_DROP,
        }
    }

    fn random_tetrimino(mut rng: StdRng) -> Tetrimino {
        Tetrimino {
            x: BOARD_WIDTH / 2,
            y: 0,
            shape: Box::new(rng.gen::<Shape>()),
            rotation: 0,
        }
    }
}

/// Tetriminos that have landed.
struct DeadTile {
    x: i32,
    y: i32,
    // this is included for the color.
    shape: Shape,
}

/// The active tetrimino.
struct Tetrimino {
    x: i32,
    y: i32,
    shape: Box<Shape>,
    rotation: i32, // clockwise rotations.
}

impl Tetrimino {
    fn draw_tetrimino(&self) -> () {
        println!("hey");
    }

    fn handle_action(&mut self, action: Action) -> () {
        let (mov_y, mov_x): (i32, i32) = action.transpose();
        let rotation: i32 = action.rotate();
        self.x += mov_x;
        self.y += mov_y;
        self.rotation += rotation;
    }
}
enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Shape {
    fn name(&self) -> char {
        match *self {
            Shape::I => 'I',
            Shape::O => 'O',
            Shape::T => 'T',
            Shape::S => 'S',
            Shape::Z => 'Z',
            Shape::J => 'J',
            Shape::L => 'L',
        }
    }
    fn color(&self) -> [f32; 4] {
        match *self {
            Shape::I => [0.95, 0.26, 0.21, 1.0], // red
            Shape::O => [0.13, 0.59, 0.95, 1.0], // blue
            Shape::T => [0.80, 0.86, 0.22, 1.0], // lime
            Shape::S => [0.00, 0.73, 0.83, 1.0], // cyan
            Shape::Z => [1.00, 0.60, 0.00, 1.0], // orange
            Shape::J => [1.00, 0.92, 0.23, 1.0], // yellow
            Shape::L => [0.91, 0.11, 0.39, 1.0], // pink (should be magenta)
        }


    }

    // tiles: an array of length 4 with relative positions to origin (top left of spawn)
    // 0.0 1.0
    // 0.1 1.1
    fn tiles(&self) -> [(i32, i32); 4] {
        match *self {
            Shape::I => [(0, 0), (1, 0), (2, 0), (3, 0)],
            Shape::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            Shape::T => [(0, 0), (1, 0), (2, 0), (1, 1)],
            Shape::S => [(0, 0), (1, 0), (-1, 1), (0, 1)],
            Shape::Z => [(0, 0), (0, 1), (1, 1), (2, 1)],
            Shape::J => [(0, 0), (1, 0), (2, 0), (2, 1)],
            Shape::L => [(0, 0), (1, 0), (2, 0), (0, 1)],
        }
    }
}

impl Rand for Shape {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let between: Range<i32> = Range::new(0, 6);
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
