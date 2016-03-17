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
use rand::{Rand, Rng};
use rand::distributions::{IndependentSample, Range};

mod constants;
use constants::*;

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

            board.render_board(c, draw_state, gl);
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
            Key::Up    => self.board.move_tetrimino(Move::ROTR),
            Key::Left  => self.board.move_tetrimino(Move::MOVL),
            Key::Down  => self.board.move_tetrimino(Move::MOVD),
            Key::Right => self.board.move_tetrimino(Move::MOVR),

            Key::W => self.board.move_tetrimino(Move::ROTR),
            Key::A => self.board.move_tetrimino(Move::MOVL),
            Key::S => self.board.move_tetrimino(Move::MOVD),
            Key::D => self.board.move_tetrimino(Move::MOVR),

            Key::Space => self.board.move_tetrimino(Move::DROP),

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


    let board = Board::initial_board();

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

fn draw_square(x: i32,
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

/// The board itself.
struct Board {
    in_progress: bool,
    dead_tiles: [[Option<DeadTile>; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize],
    tetrimino: Tetrimino,
    score: i32,
}

impl Board {
    pub fn initial_board() -> Board {
        let tetrimino = Board::random_tetrimino();
        Board {
            in_progress: true,
            dead_tiles: [[None; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize],
            tetrimino: tetrimino,
            score: 0,
        }
    }

    pub fn move_tetrimino(&mut self, mov: Move) -> () {
        if !self.in_progress {
            return ();
        }
        match mov {
            Move::ROTR => self.rotate_tetrimino(),
            Move::MOVL => self.move_tetrimino_horizontally(-1),
            Move::MOVD => self.gravity(),
            Move::MOVR => self.move_tetrimino_horizontally(1),
            Move::DROP => self.drop_tetrimino(),
        };
    }

    fn rotate_tetrimino(&mut self) -> () {
        if !self.illegal_position(self.tetrimino.tiles_rotated()) {
            self.tetrimino.rotate();
        }
    }

    fn move_tetrimino_horizontally(&mut self, distance: i32) -> () {
        if !self.illegal_position(self.tetrimino.tiles_offset((distance, 0))) {
            self.tetrimino.x += distance;
        }
    }

    fn drop_tetrimino(&mut self) -> () {
        while !self.illegal_position(self.tetrimino.tiles_offset((0, 1))) {
            self.tetrimino.y += 1;
        }
        self.tetrimino_landed();
    }

    fn gravity(&mut self) -> () {
        // is it illegal to move the tetrimino 1 tile down?
        if self.illegal_position(self.tetrimino.tiles_offset((0, 1))) {
            self.tetrimino_landed();
        } else {
            self.tetrimino.y += 1;
        }
    }

    fn tetrimino_landed(&mut self) -> () {
        // spawn dead tiles.
        let color = self.tetrimino.shape.color();
        for tile in self.tetrimino.tiles().iter() {
            self.dead_tiles[tile.0 as usize][tile.1 as usize] = Some(DeadTile { color: color });
        }
        // if loss:
        if self.tetrimino.tiles().iter().any(|t| t.1 <= 0) {
            self.in_progress = false;
        } else {
            // check if tetris achieved.
            let mut lines = 0;
            let mut highest_y = 0; // highest index, lowest (visual) line.

            'outer: for y in 0..BOARD_HEIGHT {
                if (0..BOARD_WIDTH).all(|x| self.dead_tiles[x as usize][y as usize].is_some()) {
                    lines += 1;

                    if y > highest_y {
                        highest_y = y;
                    }

                    if lines == 4 {
                        break 'outer; // 4 is the max amount of lines possible.
                    }
                }
            }

            if lines > 0 {
                // move down stuff
                for i in 0..(highest_y) {
                    let old_y = (highest_y) - i;
                    let new_y = old_y - lines;
                    for x in 0..BOARD_WIDTH {
                        let old = self.dead_tiles[x as usize][new_y as usize];

                        self.dead_tiles[x as usize][old_y as usize] = old;

                    }
                }
                // add to score.
                self.score += lines*lines*SCORE_PER_LINE;
            }

        }
        // spawn new tetrimino.
        self.tetrimino = Board::random_tetrimino();

    }
    fn illegal_position(&self, tetrimino_tiles: Vec<(i32, i32)>) -> bool {
        tetrimino_tiles.iter()
            .any(|t|
                 // too long to the left
                 t.0 < 0

                 // too long to the right
                 || t.0 == BOARD_WIDTH

                 // too low
                 || t.1 == BOARD_HEIGHT
                 // collision with dead tile
                 || self.dead_tiles[t.0 as usize][t.1 as usize].is_some())
    }

    fn render_board(&self, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        // 1. render playfield (i.e. the big rectangle where tetriminos are allowed to move.
        self.draw_playfield(c, draw_state, gl);

        // 2. render the active tetrimino
        let ref tetrimino = self.tetrimino;
        tetrimino.draw(c, draw_state, gl);

        // 3. render dead tiles
        self.draw_dead_tiles(c, draw_state, gl);
    }

    fn draw_playfield(&self, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        use graphics::*;
        let area: [f64; 4] = [(BOARD_OFFSET_X as f64) * TILE_SIZE,
                              (BOARD_OFFSET_Y as f64) * TILE_SIZE,
                              (BOARD_WIDTH as f64) * TILE_SIZE,
                              (BOARD_HEIGHT as f64) * TILE_SIZE];

        let rectangle = rectangle::Rectangle {
            color: [0.1, 0.08, 0.12, 1.0],
            shape: rectangle::Shape::Square,
            border: None,
        };

        let transform = c.transform;

        rectangle.draw(area, draw_state, transform, gl);
    }

    fn draw_dead_tiles(&self, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                for dt in (self.dead_tiles[i as usize][j as usize]).iter() {
                    draw_square(i + BOARD_OFFSET_X,
                                j + BOARD_OFFSET_Y,
                                dt.color,
                                c,
                                draw_state,
                                gl);
                }
            }
        }
    }

    fn random_tetrimino() -> Tetrimino {
        let mut rng = rand::thread_rng();
        let shape: Shape = rng.gen::<Shape>();
        Tetrimino {
            x: shape.origin(),
            y: 0,
            shape: shape,
            rotation: 0,
        }
    }
}

/// Tetriminos that have landed.
#[derive(Copy, Clone)]
struct DeadTile {
    color: [f32; 4],
}

/// The active tetrimino.
struct Tetrimino {
    x: i32,
    y: i32,
    shape: Shape,
    rotation: i32, // clockwise rotations.
}

impl Tetrimino {
    fn draw(&self, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        for tile in &self.shape.tiles(self.rotation) {
            let x = self.x + tile.0 + BOARD_OFFSET_X;
            let y = self.y + tile.1 + BOARD_OFFSET_Y;
            draw_square(x, y, self.shape.color(), c, draw_state, gl);
        }
    }

    fn tiles(&self) -> Vec<(i32, i32)> {
        self.shape
            .tiles(self.rotation)
            .iter()
            .map(|t| (t.0 + self.x, t.1 + self.y))
            .collect()
    }
    fn tiles_offset(&self, offset: (i32, i32)) -> Vec<(i32, i32)> {
        self.shape
            .tiles(self.rotation)
            .iter()
            .map(|t| (t.0 + self.x + offset.0, t.1 + self.y + offset.1))
            .collect()
    }

    fn rotate(&mut self) -> () {
        self.rotation = (self.rotation + 1) % 4;
    }

    // rotation
    fn tiles_rotated(&self) -> Vec<(i32, i32)> {
        self.shape
            .tiles((self.rotation + 1) % 4)
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

    fn origin(&self) -> i32 {
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

enum Move {
    MOVL,
    MOVR,
    MOVD,
    DROP,
    ROTR,
}
