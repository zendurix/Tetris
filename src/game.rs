use crate::block::Block;
use crate::block::BlockType;
use crate::block::MoveDir;
use crate::config;
use crate::input::Input;
use crate::map::Map;
use crate::place::Coord;
use crate::rotate;

use rand::Rng;
use sfml::graphics::{Color, RenderWindow};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

enum CollisionEffect {
    Stop,
    DontMove,
    None,
}

enum Action {
    Fall,
    FallFast,
    MoveLeft,
    MoveRight,
    Rotate,
}

pub struct Game {
    map: Map,
    blocks: Vec<Block>,
    moving_block_id: Option<usize>,
    next_block_id: usize,
    points: u32,
    game_off: bool,
}

impl Game {
    pub fn new(win: RenderWindow) -> Game {
        Game {
            map: Map::new(win),
            blocks: vec![],
            moving_block_id: None,
            next_block_id: 0,
            points: 0,
            game_off: false,
        }
    }

    pub fn game_loop(&mut self) {
        let (input_tx, input_rx): (Sender<char>, Receiver<char>) = mpsc::channel();
        let (game_off_tx, game_off_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
        let (input_read_tx, input_read_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

        let mut input_system = Input::new();
        let input_thread = thread::spawn(move || {
            input_system.input_listener_activate(input_tx, input_read_rx, game_off_rx)
        });

        // send read(true) to input system, so it will start chcecking input
        input_read_tx.send(true).unwrap();

        let mut time_counter = Instant::now();
        let fall_down_time = Duration::from_millis(config::FALL_TIME_MS);
        let mut input_received: Option<char>;

        'game_loop: loop {
            if self.moving_block_id == None {
                self.spawn_random_block();
            }

            input_received = match input_rx.try_recv() {
                Ok(inp) => {
                    input_read_tx.send(false).unwrap();
                    Some(inp)
                }
                _ => None,
            };

            match input_received {
                Some(inp) => {
                    if inp == '\n' {
                        self.game_off = true;
                    } else {
                        self.handle_input(input_received);
                        input_read_tx.send(true).unwrap();
                    }
                }
                None => (),
            }

            if time_counter.elapsed().as_millis() >= fall_down_time.as_millis() {
                self.fall_block_down();
                time_counter = Instant::now();
            }
            self.check_allignments();

            if self.game_off {
                game_off_tx.send(true).unwrap();
                match input_thread.join() {
                    Ok(_) => {
                        println!("thread closed correctly");
                    }
                    Err(_) => {
                        println!("thread closing error");
                    }
                };
                break 'game_loop;
            }
        }

        println!("GAME LOOP ENDED");
        println!("POINTS: {}", self.points);
    }

    fn handle_input(&mut self, input: Option<char>) {
        match input {
            None => (),
            Some(inp) => {
                let act = Game::input_to_action(&inp);
                self.make_action(&act);
                self.map.print_map();
            }
        }
        thread::sleep(Duration::new(0, config::TURN_TIME_NS));
    }

    fn input_to_action(act: &char) -> Action {
        match act {
            '4' => Action::MoveLeft,
            '6' => Action::MoveRight,
            '2' => Action::FallFast,
            ' ' => Action::Rotate,
            _ => Action::Fall,
        }
    }

    fn make_action(&mut self, act: &Action) {
        match act {
            Action::MoveRight => self.move_block(MoveDir::Right),
            Action::MoveLeft => self.move_block(MoveDir::Left),
            Action::Fall => self.move_block(MoveDir::Down),
            Action::Rotate => self.rotate_block(),
            Action::FallFast => self.fall_fast(),
        }
    }

    fn fall_block_down(&mut self) {
        self.move_block(MoveDir::Down);
        self.map.print_map();
    }

    fn rotate_block(&mut self) {
        match self.moving_block_id {
            None => (),
            Some(id) => {
                if let BlockType::O = self.blocks[id].get_block_type() {
                    ();
                } else {
                    let area_coord = self.blocks[id].get_block_min_xy();
                    let area = self
                        .map
                        .get_3x3_clone(area_coord.x as usize, area_coord.y as usize);
                    let rotation_coords = rotate::rotate_block(area, id);
                    match self.check_overbounding(&rotation_coords, id) {
                        CollisionEffect::None => {
                            self.unplace_block_from_map(id);
                            self.blocks[id].set_block_coords(rotation_coords);
                            self.place_block_on_map(id);
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    fn fall_fast(&mut self) {
        'fall_loop: loop {
            match self.moving_block_id {
                None => (),
                Some(id) => {
                    let move_try = self.blocks[id].try_move(&MoveDir::Down);
                    match self.check_overbounding(&move_try, id) {
                        CollisionEffect::Stop => {
                            self.moving_block_id = None;
                            break 'fall_loop;
                        }
                        _ => {
                            self.move_block(MoveDir::Down);
                        }
                    }
                }
            }
        }
        // wait for long time, so game wont drop down many blocks in row at the same time
        thread::sleep(Duration::new(0, config::TURN_TIME_NS * 5));
    }

    fn move_block(&mut self, dir: MoveDir) {
        match self.moving_block_id {
            None => (),
            Some(id) => {
                let move_try = self.blocks[id].try_move(&dir);
                match self.check_overbounding(&move_try, id) {
                    CollisionEffect::Stop => self.moving_block_id = None,
                    CollisionEffect::None => {
                        self.unplace_block_from_map(id);
                        self.blocks[id].set_block_coords(move_try);
                        self.place_block_on_map(id);
                    }
                    CollisionEffect::DontMove => (),
                }
            }
        }
    }

    fn check_overbounding(&mut self, coords: &[Coord; 4], block_id: usize) -> CollisionEffect {
        let mut y_is_0 = false;
        for coord in coords {
            if coord.y == 0 {
                y_is_0 = true;
            }
        }
        for coord in coords {
            match self.map.field[coord.x as usize][coord.y as usize].get_block_id() {
                None => (),
                Some(id) => {
                    if id == block_id {
                        ();
                    } else {
                        if y_is_0 {
                            self.game_off = true;
                            println!("GAME OVER");
                        }
                        return CollisionEffect::Stop;
                    }
                }
            }
        }

        for coord in coords {
            if coord.y as usize == config::MAP_HEIGHT - 1 {
                return CollisionEffect::Stop;
            }
            if coord.x as usize == config::MAP_LENGTH - 1 {
                return CollisionEffect::DontMove;
            }
        }

        CollisionEffect::None
    }

    fn check_allignments(&mut self) {
        let allignments = self.map.get_block_allignments();
        match allignments {
            None => (),
            Some(vec) => {
                for y in vec {
                    self.map.shift_map_down(y as i32);
                    self.points += 1;
                }
            }
        }
    }

    fn spawn_random_block(&mut self) {
        let block_type: BlockType = rand::random();
        let start_x: i32 = rand::thread_rng().gen_range(0, (config::MAP_LENGTH - 4) as i32);
        let color = Game::random_color();

        let new_block = Block::new(color, block_type, start_x);
        self.blocks.push(new_block);
        self.place_block_on_map(self.next_block_id);
        self.moving_block_id = Some(self.next_block_id as usize);
        self.next_block_id += 1;
    }

    fn unplace_block_from_map(&mut self, block_index: usize) {
        let block = &self.blocks[block_index];
        for coord in block.get_blocks_coords().iter() {
            self.map.unset_block(coord.x as usize, coord.y as usize);
        }
    }

    fn place_block_on_map(&mut self, block_index: usize) {
        let block = &self.blocks[block_index];
        for coord in block.get_blocks_coords().iter() {
            self.map.set_block(
                coord.x as usize,
                coord.y as usize,
                block_index,
                self.blocks[block_index as usize].get_color(),
            );
        }
    }

    fn random_color() -> Color {
        let col_num = rand::thread_rng().gen_range(0, 6);
        match col_num {
            0 => Color::RED,
            1 => Color::CYAN,
            2 => Color::RED,
            3 => Color::GREEN,
            4 => Color::MAGENTA,
            5 => Color::BLUE,
            _ => Color::WHITE, // won't happen
        }
    }
}
