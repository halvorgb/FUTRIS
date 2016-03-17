extern crate rand;

use opengl_graphics::{GlGraphics};
use graphics::DrawState;
use graphics::context::Context;
use self::rand::Rng;

use constants::*;
use util::*;
use model::tetrimino::*;
use model::mov::*;
use model::shape::*;

/// The board itself.
pub struct Board {
    pub in_progress: bool,
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

    pub fn move_tetrimino(&mut self, mov: Mov) -> () {
        if !self.in_progress {
            return ();
        }
        match mov {
            Mov::ROTR => self.rotate_tetrimino(),
            Mov::MOVL => self.move_tetrimino_horizontally(-1),
            Mov::MOVD => self.gravity(),
            Mov::MOVR => self.move_tetrimino_horizontally(1),
            Mov::DROP => self.drop_tetrimino(),
        };
    }

    pub fn rotate_tetrimino(&mut self) -> () {
        if !self.illegal_position(self.tetrimino.tiles_rotated()) {
            self.tetrimino.rotate();
        }
    }

    pub fn move_tetrimino_horizontally(&mut self, distance: i32) -> () {
        if !self.illegal_position(self.tetrimino.tiles_offset((distance, 0))) {
            self.tetrimino.x += distance;
        }
    }

    pub fn drop_tetrimino(&mut self) -> () {
        while !self.illegal_position(self.tetrimino.tiles_offset((0, 1))) {
            self.tetrimino.y += 1;
        }
        self.tetrimino_landed();
    }

    pub fn gravity(&mut self) -> () {
        // is it illegal to move the tetrimino 1 tile down?
        if self.illegal_position(self.tetrimino.tiles_offset((0, 1))) {
            self.tetrimino_landed();
        } else {
            self.tetrimino.y += 1;
        }
    }

    pub fn tetrimino_landed(&mut self) -> () {
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
    pub fn illegal_position(&self, tetrimino_tiles: Vec<(i32, i32)>) -> bool {
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

    pub fn render_board(&self, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
        // 1. render playfield (i.e. the big rectangle where tetriminos are allowed to move.
        self.draw_playfield(c, draw_state, gl);

        // 2. render the active tetrimino
        let ref tetrimino = self.tetrimino;
        tetrimino.draw(c, draw_state, gl);

        // 3. render dead tiles
        self.draw_dead_tiles(c, draw_state, gl);
    }

    pub fn draw_playfield(&self, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
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

    pub fn draw_dead_tiles(&self, c: Context, draw_state: &DrawState, gl: &mut GlGraphics) -> () {
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

    pub fn random_tetrimino() -> Tetrimino {
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
