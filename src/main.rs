mod block;
mod config;
mod game;
mod map;
mod place;
mod rotate;
use crate::place::Place;

extern crate rand;

use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

struct GameState;
impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        Ok(())
    }
}

fn main() {
    //-> tetra::Result {

    let arr = [
        [Place::new(7, 4), Place::new(8, 4), Place::new(9, 4)],
        [Place::new(7, 5), Place::new(8, 5), Place::new(9, 5)],
        [Place::new(7, 6), Place::new(8, 6), Place::new(9, 6)],
    ];

    rotate::rotate_block(&arr);

    // ContextBuilder::new(" TETRIS ", config::WIN_LENGTH, config::WIN_HEIGHT)
    //     .build()?
    //    .run(|_| Ok(GameState))
}
