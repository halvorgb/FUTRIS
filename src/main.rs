extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::DrawState;

mod constants;
mod util;
mod model;
use model::board::Board;
use model::mov::Mov;
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
            Key::Up    => self.board.move_tetrimino(Mov::ROTR),
            Key::Left  => self.board.move_tetrimino(Mov::MOVL),
            Key::Down  => self.board.move_tetrimino(Mov::MOVD),
            Key::Right => self.board.move_tetrimino(Mov::MOVR),

            Key::W => self.board.move_tetrimino(Mov::ROTR),
            Key::A => self.board.move_tetrimino(Mov::MOVL),
            Key::S => self.board.move_tetrimino(Mov::MOVD),
            Key::D => self.board.move_tetrimino(Mov::MOVR),

            Key::Space => self.board.move_tetrimino(Mov::DROP),

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
