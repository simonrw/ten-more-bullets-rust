extern crate sdl2;
extern crate time;

mod gun;
mod entity;
mod game;

fn main() {
    let mut game = game::Game::new();
    game.run();
}