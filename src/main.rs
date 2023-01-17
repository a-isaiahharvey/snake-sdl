#![warn(rust_2018_idioms, clippy::all)]

use game::Game;
use log::{info, SetLoggerError};
use renderer::Renderer;

pub mod game;
pub mod renderer;
pub mod snake;

fn main() -> Result<(), SetLoggerError> {
    pretty_env_logger::init();

    let mut renderer = Renderer::new();

    let mut game = Game::new(32, 32);

    game.run(&mut renderer, 1000 / 60);
    info!("Score: {}", game.get_score());
    info!("Size: {}", game.get_size());
    Ok(())
}
