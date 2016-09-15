extern crate piston_window;
extern crate rand;
extern crate find_folder;
extern crate graphics;
use piston_window::*;
mod game;
mod bird;

fn main() {
    let mut game = game::Game::new();
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Pelican't", (game.width, game.height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    game.run(&mut window);
}
