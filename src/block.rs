use crate::config;
use crate::place::Coord;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use sfml::graphics::Color;

#[derive(Clone, Copy, std::fmt::Debug)]
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
            _ => BlockType::O, // this won't happen, cause all rng output is handled
        }
    }
}

pub enum MoveDir {
    Down,
    Left,
    Right,
}

/*  MDELETE
// derive for comparison
#[derive(PartialEq, Eq)]
pub enum RotateState {
    Up = 1,
    Right = 2,
    Down = 3,
    Left = 4,
}

*/

pub struct Block {
    squares: [Coord; 4], // positions for 4 blocks within Block
    color: Color,        //RGB color
    block_type: BlockType,
}

impl Block {
    pub fn new(color: Color, block_type: BlockType, start_x: i32) -> Block {
        assert!(
            (start_x as usize) < config::MAP_LENGTH - 4,
            "Block spawned out of bounds"
        );

        Block {
            squares: Block::make_squares_coords(&block_type, start_x),
            block_type,
            color,
        }
    }

    pub fn try_move(&self, dir: &MoveDir) -> [Coord; 4] {
        let mut coords = [
            Coord::new(0, 0),
            Coord::new(0, 0),
            Coord::new(0, 0),
            Coord::new(0, 0),
        ];
        let transform: (i32, i32) = match dir {
            MoveDir::Down => (0, 1),
            MoveDir::Right => (1, 0),
            MoveDir::Left => (-1, 0),
        };

        let mut dont_move_left = false;

        for square in self.squares.iter() {
            if square.x == 0 {
                dont_move_left = true;
            }
        }

        for (i, coord) in coords.iter_mut().enumerate() {
            if dont_move_left && transform.0 == -1 {
                coord.x = self.squares[i].x;
            } else {
                coord.x = self.squares[i].x + transform.0;
            }
            coord.y = self.squares[i].y + transform.1;
        }
        coords
    }

    pub fn get_block_min_xy(&self) -> Coord {
        let mut min_x = 100;
        let mut min_y = 100;
        let mut squares_y: Vec<i32> = vec![];
        for square in self.squares.iter() {
            if !squares_y.contains(&square.y) {
                squares_y.push(square.y);
            }
            if square.x < min_x {
                min_x = square.x;
            }
            if square.y < min_y {
                min_y = square.y;
            }
        }

        if squares_y.len() < 3 && min_y != 0 {
            min_y -= 1;
        }

        Coord::new(min_x, min_y)
    }

    fn make_squares_coords(block_type: &BlockType, start_x: i32) -> [Coord; 4] {
        match block_type {
            BlockType::I => Block::make_square_coords_i(start_x),
            BlockType::O => Block::make_square_coords_o(start_x),
            BlockType::T => Block::make_square_coords_t(start_x),
            BlockType::S => Block::make_square_coords_s(start_x),
            BlockType::Z => Block::make_square_coords_z(start_x),
            BlockType::J => Block::make_square_coords_j(start_x),
            BlockType::L => Block::make_square_coords_l(start_x),
        }
    }

    fn make_square_coords_i(start_x: i32) -> [Coord; 4] {
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

    fn make_square_coords_o(start_x: i32) -> [Coord; 4] {
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

    fn make_square_coords_t(start_x: i32) -> [Coord; 4] {
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

    fn make_square_coords_s(start_x: i32) -> [Coord; 4] {
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

    fn make_square_coords_z(start_x: i32) -> [Coord; 4] {
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

    fn make_square_coords_j(start_x: i32) -> [Coord; 4] {
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

    fn make_square_coords_l(start_x: i32) -> [Coord; 4] {
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

// setters / getters
impl Block {
    pub fn get_blocks_coords(&self) -> [Coord; 4] {
        self.squares.clone()
    }

    pub fn set_block_coords(&mut self, coords: [Coord; 4]) {
        self.squares = coords;
    }

    pub fn get_color(&self) -> Color {
        self.color.clone()
    }

    pub fn get_block_type(&self) -> BlockType {
        self.block_type
    }
}
