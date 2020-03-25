use crate::block::Block;
use crate::block::BlockType;
use crate::block::MoveDir;
use crate::map::Map;
use crate::place::Coord;
use crate::place::Place;
use crate::config;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use tetra::graphics::{self, Color};

enum CollisionEffect {
    stop,
    dont_move,
    none,
}

struct Game {
    map: Map,
    blocks: Vec<Block>,
    moving_block_id: Option<usize>,
    next_block_id: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            map: Map::new(),
            blocks: vec![],
            moving_block_id: None,
            next_block_id: 0,
        }
    }

    pub fn move_block(&self, dir: MoveDir) {
        match self.moving_block_id {
            None => unimplemented!(),
            Some(id) => {
                let move_try = self.blocks[id].try_move(&dir);
            }
        }
    }

    pub fn check_overbounding(&self, coords: &[Coord; 4]) -> CollisionEffect {
        for coord in coords {
            if self.map.field[coord.x as usize][coord.y as usize].block_id != None {
                return CollisionEffect::stop;
            }
        }

        CollisionEffect::none
    }

    pub fn spawn_random_block(&mut self) {
        let block_type: BlockType = rand::random();
        let start_x: i8 = rand::thread_rng().gen_range(0, (config::MAP_LENGTH-4) as i8);
        let color = Game::random_color();

        let new_block = Block::new(self.next_block_id, color, block_type, start_x);

        self.next_block_id += 1;
    }

    fn random_color() -> tetra::graphics::Color {
        let col_num = rand::thread_rng().gen_range(0, 4);
        match col_num {
            0 => tetra::graphics::Color::RED,
            1 => tetra::graphics::Color::GREEN,
            2 => tetra::graphics::Color::BLUE,
            // >= 3
            _ => tetra::graphics::Color::WHITE,
        }
    }

}
