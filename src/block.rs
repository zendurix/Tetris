use crate::config;
use crate::place::Coord;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub enum BlockType {
    // block are tetronimos. names are taken from https://tetris.wiki/Tetromino
    I, // ####
    ////
    O, // ##
    ///// ##
    ////
    T, //  #
    ///// ###
    ////
    S, //  ##
    ///// ##
    ////
    Z, // ##
    /////  ##
    ////
    J, // #
    ///// ###
    ////
    L, /////   #
       ///// ###
}

// this trait allows to use random on BlockType
impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        match rng.gen_range(0, 7) {
            0 => BlockType::I,
            1 => BlockType::O,
            2 => BlockType::T,
            3 => BlockType::S,
            4 => BlockType::Z,
            5 => BlockType::J,
            6 => BlockType::L,
            _ => BlockType::L, // this won't happen, cause all rng output is handled
        }
    }
}



pub enum MoveDir {
    down,
    left,
    right,
}

// derive for comparison
#[derive(PartialEq, Eq)]
pub enum RotateState {
    up = 1,
    right = 2,
    down = 3,
    left = 4,
}

pub struct Block {
    blocks: [Coord; 4], // positions for 4 blocks within Block
    color: tetra::graphics::Color,     //RGB color
    is_moving: bool,
    block_type: BlockType,
    rotate_state: RotateState,
    id: i32,
}

// construction
impl Block {
    pub fn new(id: i32, color: tetra::graphics::Color, block_type: BlockType, start_x: i8) -> Block {
        assert!(
            (start_x as usize) < config::MAP_LENGTH - 4,
            "Block spawned out of bounds"
        );

        Block {
            blocks: Block::make_blocks_coords(&block_type, start_x),
            rotate_state: RotateState::right,
            is_moving: true,
            block_type,
            color,
            id,
        }
    }

    pub fn try_move(&self, dir: &MoveDir) -> [Coord; 4] {
        let mut coords = [
            Coord { x: -1, y: -1 },
            Coord { x: -1, y: -1 },
            Coord { x: -1, y: -1 },
            Coord { x: -1, y: -1 },
        ];
        let transform: Coord = match dir {
            MoveDir::down => Coord { x: 0, y: 1 },
            MoveDir::right => Coord { x: 1, y: 0 },
            MoveDir::left => Coord { x: -1, y: 0 },
        };
        for (i, coord) in coords.iter_mut().enumerate() {
            coord.x = self.blocks[i].x + transform.x;
            coord.y = self.blocks[i].y + transform.y;
        }
        coords
    }

    fn make_blocks_coords(block_type: &BlockType, start_x: i8) -> [Coord; 4] {
        match block_type {
            BlockType::I => Block::make_blocks_coords_I(start_x),
            BlockType::O => Block::make_blocks_coords_O(start_x),
            BlockType::T => Block::make_blocks_coords_T(start_x),
            BlockType::S => Block::make_blocks_coords_S(start_x),
            BlockType::Z => Block::make_blocks_coords_Z(start_x),
            BlockType::J => Block::make_blocks_coords_J(start_x),
            BlockType::L => Block::make_blocks_coords_L(start_x),
        }
    }

    fn make_blocks_coords_I(start_x: i8) -> [Coord; 4] {
        [
            Coord { x: start_x, y: 0 },
            Coord {
                x: start_x + 1,
                y: 0,
            },
            Coord {
                x: start_x + 2,
                y: 0,
            },
            Coord {
                x: start_x + 3,
                y: 0,
            },
        ]
    }

    fn make_blocks_coords_O(start_x: i8) -> [Coord; 4] {
        [
            Coord { x: start_x, y: 0 },
            Coord {
                x: start_x + 1,
                y: 0,
            },
            Coord { x: start_x, y: 1 },
            Coord {
                x: start_x + 1,
                y: 1,
            },
        ]
    }

    fn make_blocks_coords_T(start_x: i8) -> [Coord; 4] {
        [
            Coord {
                x: start_x + 1,
                y: 0,
            },
            Coord { x: start_x, y: 1 },
            Coord {
                x: start_x + 1,
                y: 1,
            },
            Coord {
                x: start_x + 2,
                y: 1,
            },
        ]
    }

    fn make_blocks_coords_S(start_x: i8) -> [Coord; 4] {
        [
            Coord {
                x: start_x + 1,
                y: 0,
            },
            Coord {
                x: start_x + 2,
                y: 0,
            },
            Coord { x: start_x, y: 1 },
            Coord {
                x: start_x + 1,
                y: 1,
            },
        ]
    }

    fn make_blocks_coords_Z(start_x: i8) -> [Coord; 4] {
        [
            Coord { x: start_x, y: 0 },
            Coord {
                x: start_x + 1,
                y: 0,
            },
            Coord {
                x: start_x + 1,
                y: 1,
            },
            Coord {
                x: start_x + 2,
                y: 1,
            },
        ]
    }

    fn make_blocks_coords_J(start_x: i8) -> [Coord; 4] {
        [
            Coord { x: start_x, y: 0 },
            Coord { x: start_x, y: 1 },
            Coord {
                x: start_x + 1,
                y: 1,
            },
            Coord {
                x: start_x + 2,
                y: 1,
            },
        ]
    }

    fn make_blocks_coords_L(start_x: i8) -> [Coord; 4] {
        [
            Coord {
                x: start_x + 2,
                y: 0,
            },
            Coord { x: start_x, y: 1 },
            Coord {
                x: start_x + 1,
                y: 1,
            },
            Coord {
                x: start_x + 2,
                y: 1,
            },
        ]
    }
}

/*
    fn rotate_I(blocks: &[Coord; 4], rotate_state: &RotateState) -> [Coord; 4] {
        if let RotateState::left | RotateState::right = rotate_state {
            return [
                Coord{x: blocks[0].x + 2, y: blocks[0].y - 2},
                Coord{x: blocks[0].x + 2, y: blocks[0].y - 1},
                Coord{x: blocks[0].x + 2, y: blocks[0].y - 0},
                Coord{x: blocks[0].x + 2, y: blocks[0].y + 1}
            ];
        }
        else { // lup || down
            return [
                Coord{x: blocks[0].x - 2, y: blocks[0].y + 2},
                Coord{x: blocks[0].x - 1, y: blocks[0].y + 2},
                Coord{x: blocks[0].x    , y: blocks[0].y + 2},
                Coord{x: blocks[0].x + 1, y: blocks[0].y + 2}
            ];
        }
    }

    fn rotate_O(blocks: &[Coord; 4], rotate_state: &RotateState) -> [Coord; 4] {
        *blocks   // square can't rotate
    }
*/
