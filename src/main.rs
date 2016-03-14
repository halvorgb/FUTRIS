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
const BOARD_OFFSET_X: i32 = 2;
const BOARD_OFFSET_Y: i32 = 1;
const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 30;
const INITIAL_MS_PER_DROP: f32 = 10.0;

pub struct Futris {
    gl: GlGraphics, // OpenGL drawing backend.
    draw_state: DrawState,
    background_color: [f32; 4],
    board: Board, // the game state
}
impl Futris {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let bgc = self.background_color;
        let ref board = self.board;
        let ref draw_state = self.draw_state;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(bgc, gl);

            board.render_board(TILE_SIZE, BOARD_WIDTH, BOARD_HEIGHT, c, draw_state, gl);
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


    let rng = rand::thread_rng();
    //let seed: &[_] = &[initial_rng.gen::<i32>(), initial_rng.gen::<i32>()];
    //let mut rng: StdRng = SeedableRng::from_seed(seed);
    let board = Board::initial_board(BOARD_OFFSET_X, BOARD_OFFSET_Y, BOARD_WIDTH, BOARD_HEIGHT, rng);
    // Create a new game and run it.
    let mut futris = Futris {
        gl: GlGraphics::new(opengl),
        draw_state: DrawState::new(),
        background_color: [0.06, 0.04, 0.08, 1.0],
        board: board,
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            futris.render(&r);
        }

        if let Some(u) = e.update_args() {
            futris.update(&u);
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
    offset_x: i32,
    offset_y: i32,
    width: i32,
    height: i32,
}

impl Board {
    fn move_tetrimino(&mut self) -> () {
        println!("MOVE!")
    }

    fn initial_board<R: Rng>(offset_x: i32, offset_y: i32, width: i32, height: i32, rng: R) -> Board {
        let initial_dead_tiles = vec![
            Box::new(DeadTile {
                x: 0,
                y: 2,
                shape: Shape::I,
            }),
            Box::new(DeadTile {
                x: 1,
                y: 2,
                shape: Shape::I,
            }),

            Box::new(DeadTile {
                x: 2,
                y: 2,
                shape: Shape::I,
            }),

            Box::new(DeadTile {
                x: 3,
                y: 2,
                shape: Shape::I,
            }),

            Box::new(DeadTile {
                x: 4,
                y: 2,
                shape: Shape::I,
            }),

            Box::new(DeadTile {
                x: 5,
                y: 2,
                shape: Shape::I,
            }),

            Box::new(DeadTile {
                x: 6,
                y: 2,
                shape: Shape::I,
            }),
            Box::new(DeadTile {
                x: 7,
                y: 2,
                shape: Shape::I,
            }),
            Box::new(DeadTile {
                x: 8,
                y: 2,
                shape: Shape::I,
            }),
            Box::new(DeadTile {
                x: 9,
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
                x: 9,
                y: 12,
                shape: Shape::T,
            }),
            Box::new(DeadTile {
                x: 8,
                y: 14,
                shape: Shape::Z,
            }),
        ];

        Board {
            dead_tiles: initial_dead_tiles,
            tetrimino: Board::random_tetrimino(width, rng),
            ms_per_drop: INITIAL_MS_PER_DROP,
            offset_x: offset_x,
            offset_y: offset_y,
            width: width,
            height: height,
        }
    }

    fn render_board(&self, tile_size: i32, board_width: i32, board_height: i32, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        // 1. render playfield (i.e. the big rectangle where tetriminos are allowed to move.
        self.draw_playfield(tile_size, board_width, board_height, c, draw_state, gl);

        // 2. render the active tetrimino
        let ref tetrimino = self.tetrimino;
        tetrimino.draw(self.offset_x, self.offset_y, TILE_SIZE, c, draw_state, gl);

        // 3. render dead tiles
        let ref dead_tiles: Vec<Box<DeadTile>> = self.dead_tiles;
        for dead_tile in dead_tiles {
            dead_tile.draw(self.offset_x, self.offset_y, TILE_SIZE, c, draw_state, gl);
        }

    }

    fn draw_playfield(&self, tile_size: i32, board_width: i32, board_height: i32, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        use graphics::*;
        let area: [f64; 4] = [(self.offset_x * tile_size) as f64,
                              (self.offset_y * tile_size) as f64,
                              (board_width * tile_size) as f64,
                              (board_height * tile_size) as f64];

        let rectangle = rectangle::Rectangle {
            color: [0.1, 0.08, 0.12, 1.0],
            shape: rectangle::Shape::Square,
            border: None,
        };

        let transform = c.transform;

        rectangle.draw(area, draw_state, transform, gl);
    }

    fn random_tetrimino<R: Rng>(width: i32, mut rng: R) -> Tetrimino {
        let shape: Box<Shape> = Box::new(rng.gen::<Shape>());
        Tetrimino {
            x: shape.origin(width),
            y: 0,
            shape: shape,
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
   fn draw(&self, offset_x: i32, offset_y: i32, tile_size: i32, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
       draw_square(tile_size, self.x + offset_x, self.y + offset_y, self.shape.color(), c, draw_state, gl);
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
    fn draw(&self, offset_x: i32, offset_y: i32, tile_size: i32, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        use graphics::*;
        for tile in &self.shape.tiles() {
            let x = self.x + tile.0 + offset_x;
            let y = self.y + tile.1 + offset_y;
            draw_square(tile_size, x, y, self.shape.color(), c, draw_state, gl);
        }
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

    fn origin(&self, board_width: i32) -> i32 {
        match *self {
            Shape::I => board_width/2 - 2,
            Shape::O => board_width/2 - 2,
            Shape::T => board_width/2 - 2,
            Shape::S => board_width/2 - 1,
            Shape::Z => board_width/2 - 2,
            Shape::J => board_width/2 - 2,
            Shape::L => board_width/2 - 2,
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
