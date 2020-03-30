mod block;
mod config;
mod game;
mod input;
mod map;
mod place;
mod rotate;

use crate::game::Game;
extern crate rand;
extern crate sfml;

use sfml::{
    graphics::RenderWindow,
    window::{ContextSettings, Style},
};
use std::io::{stdin, Read};

fn main() {
    let context_settings = ContextSettings {
        antialiasing_level: 0,
        ..Default::default()
    };
    let window = RenderWindow::new(
        (config::WIN_LENGTH as u32, config::WIN_HEIGHT as u32),
        "TETRIS",
        Style::CLOSE,
        &context_settings,
    );

    let mut game = Game::new(window);

    game.game_loop();

    println!("press enter to exit");
    stdin().read(&mut [0u8]).unwrap();
}
