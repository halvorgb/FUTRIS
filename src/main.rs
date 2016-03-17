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
use rand::{Rand, Rng, SeedableRng, ThreadRng};
use rand::distributions::{IndependentSample, Range};

const WINDOW_TITLE: &'static str = "FUTRIS";
const TILE_SIZE: i32 = 32;
const BOARD_OFFSET_X: i32 = 2;
const BOARD_OFFSET_Y: i32 = 2;
const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 30;
const INITIAL_S_PER_DROP: f64 = 0.10;
const MINIMUM_S_PER_DROP: f64 = 0.05;

pub struct Futris {
    gl: GlGraphics, // OpenGL drawing backend.
    draw_state: DrawState,
    background_color: [f32; 4],
    board: Board, // the game state
    lag: f64,
    s_per_drop: f64,
}

impl Futris {
    fn render(&mut self, args: &RenderArgs) -> () {
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

    fn update(&mut self, args: &UpdateArgs) -> () {
        self.lag += args.dt;
        if self.board.in_progress {
            if self.lag > self.s_per_drop {
                self.lag -= self.s_per_drop;
                self.board.gravity();
            }
        }
    }

    fn handle_key_input(&mut self, key: keyboard::Key) -> () {
        match key {
            Key::Up => self.board.rotate_tetrimino(),
            Key::Left => self.board.move_tetrimino(-1),
            Key::Down => self.board.gravity(),
            Key::Right => self.board.move_tetrimino(1),

            Key::W => self.board.rotate_tetrimino(),
            Key::A => self.board.move_tetrimino(-1),
            Key::S => self.board.gravity(),
            Key::D => self.board.move_tetrimino(1),

            Key::R => println!("HEY, lets restart!"),
            _ => println!("hey"),
        }
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


    let rng: ThreadRng = rand::thread_rng();
    let board = Board::initial_board(BOARD_OFFSET_X, BOARD_OFFSET_Y, BOARD_WIDTH, BOARD_HEIGHT, rng);

    // Create a new game and run it.
    let mut futris = Futris {
        gl: GlGraphics::new(opengl),
        draw_state: DrawState::new(),
        background_color: [0.06, 0.04, 0.08, 1.0],
        board: board,
        lag: 0.0,
        s_per_drop: INITIAL_S_PER_DROP,
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            futris.render(&r);
        }

        if let Some(u) = e.update_args() {
            futris.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            futris.handle_key_input(key);
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

/// The board itself.
struct Board {
    in_progress: bool,
    dead_tiles: Vec<Box<DeadTile>>,
    tetrimino: Tetrimino,
    points: i32,
    offset_x: i32,
    offset_y: i32,
    width: i32,
    height: i32,
}

impl Board {
    fn rotate_tetrimino(&mut self) -> () {
        if !self.illegal_position(self.tetrimino.tiles_rotated()) {
            self.tetrimino.rotate();
        }
    }

    fn move_tetrimino(&mut self, distance: i32) -> () {
        if !self.illegal_position(self.tetrimino.tiles_offset((distance, 0))) {
            self.tetrimino.x += distance;
        }
    }

    fn gravity(&mut self) -> () {
        if !self.in_progress {
            return ()
        }

        // is it illegal to move the tetrimino 1 tile down?
        if self.illegal_position(self.tetrimino.tiles_offset((0,1))) {
            let color = self.tetrimino.shape.color();
            for tile in self.tetrimino.tiles().iter() {
                self.dead_tiles.push(Box::new(DeadTile {
                    x: tile.0,
                    y: tile.1,
                    color: color,
                }));
            }
            // if loss:
            if self.tetrimino.y <= 0 {
                self.in_progress = false;
            }

            self.tetrimino = Board::random_tetrimino(self.width);

        } else {
            self.tetrimino.y += 1;
        }
    }

    fn illegal_position(&self, tetrimino_tiles: Vec<(i32, i32)>) -> bool {
        tetrimino_tiles.iter().any(
            |t| self.dead_tiles.iter().any(
                |d| t.0 == d.x && t.1 == d.y) // collision with dead tile

                || t.0 < 0 // too long to the left
                || t.0 == self.width // too long to the right
                || t.1 == self.height) // too low
    }

    fn initial_board(offset_x: i32, offset_y: i32, width: i32, height: i32, rng: ThreadRng) -> Board {
        let tetrimino = Board::random_tetrimino(width);
        Board {
            in_progress: true,
            dead_tiles: Vec::new(),
            tetrimino: tetrimino,
            points: 0,
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

    fn random_tetrimino(width: i32) -> Tetrimino {

        let mut rng = rand::thread_rng();
        let shape: Shape = rng.gen::<Shape>();
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
    color: [f32; 4],
}

impl DeadTile {
   fn draw(&self, offset_x: i32, offset_y: i32, tile_size: i32, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
       draw_square(tile_size, self.x + offset_x, self.y + offset_y, self.color, c, draw_state, gl);
    }
 }

/// The active tetrimino.
struct Tetrimino {
    x: i32,
    y: i32,
    shape: Shape,
    rotation: i32, // clockwise rotations.
}

impl Tetrimino {
    fn draw(&self, offset_x: i32, offset_y: i32, tile_size: i32, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        use graphics::*;
        for tile in &self.shape.tiles(self.rotation) {
            let x = self.x + tile.0 + offset_x;
            let y = self.y + tile.1 + offset_y;
            draw_square(tile_size, x, y, self.shape.color(), c, draw_state, gl);
        }
    }

    fn tiles(&self) -> Vec<(i32, i32)> {
        self.shape.tiles(self.rotation)
            .iter()
            .map(|t| (t.0 + self.x, t.1 + self.y))
            .collect()
    }
    fn tiles_offset(&self, offset: (i32, i32)) -> Vec<(i32, i32)> {
        self.shape.tiles(self.rotation)
            .iter()
            .map(|t| (t.0 + self.x + offset.0, t.1 + self.y + offset.1))
            .collect()
    }

    fn rotate(&mut self) -> () {
        self.rotation = (self.rotation + 1) % 4;
    }

    // rotation
    fn tiles_rotated(&self) -> Vec<(i32, i32)> {
        self.shape.tiles((self.rotation+1) % 4)
            .iter()
            .map(|t| (t.0 + self.x, t.1 + self.y))
            .collect()
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
    fn copy(&self) -> Shape {
        match *self {
            Shape::I => Shape::I,
            Shape::O => Shape::O,
            Shape::T => Shape::T,
            Shape::S => Shape::S,
            Shape::Z => Shape::Z,
            Shape::J => Shape::J,
            Shape::L => Shape::L,
        }
    }

    fn color(&self) -> [f32; 4] {
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

    fn tiles(&self, rotation: i32) -> Vec<(i32, i32)> {
        match *self {
            Shape::I => match rotation {
                0 => vec![(0, 1), (1, 1), (2, 1), (3, 1)],
                1 => vec![(2, 0), (2, 1), (2, 2), (2, 3)],
                2 => vec![(0, 2), (1, 2), (2, 2), (3, 2)],
                _ => vec![(1, 0), (1, 1), (1, 2), (1, 3)],
            },
            Shape::O => match rotation {
                _ => vec![(1, 0), (2, 0),(1, 1), (2,1)],
            },

            Shape::T => match rotation {
                0 => vec![(1, 0), (0, 1), (1, 1), (2, 1)],
                1 => vec![(1, 0), (1, 1), (1, 2), (2, 1)],
                2 => vec![(0, 1), (1, 1), (2, 1), (1, 2)],
                _ => vec![(1, 0), (1, 1), (1, 2), (0, 1)],
            },

            Shape::S => match rotation {
                0 => vec![(1, 0), (2, 0), (0, 1), (1, 1)],
                1 => vec![(1, 0), (1, 1), (2, 1), (2, 2)],
                2 => vec![(1, 1), (2, 1), (0, 2), (1, 2)],
                _ => vec![(0, 0), (0, 1), (1, 1), (1, 2)],
            },

            Shape::Z => match rotation {
                0 => vec![(0, 0), (1, 0), (1, 1), (2, 1)],
                1 => vec![(2, 0), (2, 1), (1, 1), (1, 2)],
                2 => vec![(0, 1), (1, 1), (1, 2), (2, 2)],
                _ => vec![(1, 0), (1, 1), (0, 1), (0, 2)],
            },

            Shape::J => match rotation {
                0 => vec![(0, 0), (0, 1), (1, 1), (2, 1)],
                1 => vec![(1, 0), (2, 0), (1, 1), (1, 2)],
                2 => vec![(0, 1), (1, 1), (2, 1), (2, 2)],
                _ => vec![(2, 0), (2, 1), (2, 2), (1, 2)],
            },

            Shape::L => match rotation {
                0 => vec![(2, 0), (0, 1), (1, 1), (2, 1)],
                1 => vec![(1, 0), (1, 1), (1, 2), (2, 2)],
                2 => vec![(0, 1), (1, 1), (2, 1), (0, 2)],
                _ => vec![(0, 0), (1, 0), (1, 1), (1, 2)],
            },
        }
    }

    fn rotate_tuple(tuple: (i32, i32), n: i32) -> (i32, i32) {
        (0..n).fold(tuple, |t, _| (t.1, -t.0))
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
