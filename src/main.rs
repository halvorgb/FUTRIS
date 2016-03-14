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
use graphics::DrawState;
use graphics::context::Context;
use rand::{Rand, Rng, SeedableRng, StdRng};
use rand::distributions::{IndependentSample, Range};

const WINDOW_TITLE: &'static str = "FUTRIS";
const TILE_SIZE: i32 = 32;
const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 30;
const INITIAL_MS_PER_DROP: f32 = 10.0;

pub struct Tetris {
    gl: GlGraphics, // OpenGL drawing backend.
    draw_state: DrawState,
    background_color: [f32; 4],
    board: Board, // the game state
}
impl Tetris {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let bgc = self.background_color;
        let ref board = self.board;
        let ref draw_state = self.draw_state;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(bgc, gl);

            board.render_board(TILE_SIZE, c.transform, c, draw_state, gl);
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
    let board = Board::initial_board(rng);
    // Create a new game and run it.
    let mut tetris = Tetris {
        gl: GlGraphics::new(opengl),
        draw_state: DrawState::new(),
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

fn draw_square(tile_size: i32, x: i32, y: i32, color: [f32; 4],
             c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
    use graphics::*;
    let square = rectangle::square(0.0, 0.0, tile_size as f64);
    let border = rectangle::Border{
        color: [0.0, 0.0, 0.0, 1.0],
        radius: 1.0,
    };

    let rectangle = rectangle::Rectangle {
        color: color,
        shape: rectangle::Shape::Bevel(4.0),
        border: Some(border),
    };

    let transform = c.transform
        .trans((x * tile_size) as f64, (y * tile_size) as f64);

    rectangle.draw(square, draw_state, transform, gl);
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
    ms_per_drop: f32,
}

impl Board {
    fn move_tetrimino(&mut self) -> () {
        println!("MOVE!")
    }

    fn initial_board(rng: StdRng) -> Board {
        let initial_dead_tiles = vec![
            Box::new(DeadTile {
                x: 0,
                y: 2,
                shape: Shape::I,
            }),
            Box::new(DeadTile {
                x: 2,
                y: 4,
                shape: Shape::J,
            }),
            Box::new(DeadTile {
                x: 4,
                y: 6,
                shape: Shape::L,
            }),
            Box::new(DeadTile {
                x: 6,
                y: 8,
                shape: Shape::O,
            }),
            Box::new(DeadTile {
                x: 8,
                y: 10,
                shape: Shape::S,
            }),
            Box::new(DeadTile {
                x: 10,
                y: 12,
                shape: Shape::T,
            }),
            Box::new(DeadTile {
                x: 12,
                y: 14,
                shape: Shape::Z,
            }),
        ];

        Board {
            dead_tiles: initial_dead_tiles,
            tetrimino: Board::random_tetrimino(rng),
            ms_per_drop: INITIAL_MS_PER_DROP,
        }
    }

    fn render_board(&self, tile_size: i32, global_transform: graphics::math::Matrix2d, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        // 1. render playfield (i.e. the big rectangle where tetriminos are allowed to move.
        // 2. render the active tetrimino

        // 3. render dead tiles
        let ref dead_tiles: Vec<Box<DeadTile>> = self.dead_tiles;
        for dead_tile in dead_tiles {
            dead_tile.draw(TILE_SIZE, c.transform, c, draw_state, gl);
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

impl DeadTile {
   fn draw(&self, tile_size: i32, global_transform: graphics::math::Matrix2d, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
       draw_square(tile_size, self.x, self.y, self.shape.color(), c, draw_state, gl);
    }
 }

/// The active tetrimino.
struct Tetrimino {
    x: i32,
    y: i32,
    shape: Box<Shape>,
    rotation: i32, // clockwise rotations.
}

impl Tetrimino {
    fn draw(&self, tile_size: i32, global_transform: graphics::math::Matrix2d, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        use graphics::*;
    }
    // fn to_rects(&self, tile_width: i32, tile_heigth:i32) -> graphics::rectangle::Rectangle {
    //     self.shape.tiles()
    //     graphics::rectangle::Rectangle {
    //         color: self.shape.color(),
    //         shape: graphics::rectangle::Shape::Square,
    //         border: None,
    //     }
    // }

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
