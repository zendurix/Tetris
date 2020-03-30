// constant values, that will be used all around project

pub const MAP_HEIGHT: usize = 25;  // 24
pub const MAP_LENGTH: usize = 11;  // 10 
pub const BLOCK_SIZE: usize = 30;  // in pixels

pub const WIN_HEIGHT: usize = (MAP_HEIGHT - 1) * BLOCK_SIZE;
pub const WIN_LENGTH: usize = (MAP_LENGTH - 1) * BLOCK_SIZE;

// delay between handling user input -  0.1 sex
pub const TURN_TIME_NS: u32 = 100000000;   
// delay between block moving 1 place down  -  0.5 sec
pub const FALL_TIME_MS: u64 = 500;   
