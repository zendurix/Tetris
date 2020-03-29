use crate::config;
use crate::place::Place;

use sfml::graphics::{Color, RenderTarget, RenderWindow};

pub struct Map {
    win: RenderWindow, // SFML Window
    pub field: Vec<Vec<Place>>,
}

impl Map {
    pub fn new(win: RenderWindow) -> Map {
        let field = Map::init_field();
        Map { field, win }
    }

    fn init_field() -> Vec<Vec<Place>> {
        let mut field: Vec<Vec<Place>> = vec![];
        for x in 0..config::MAP_LENGTH as i32 {
            let mut field_row: Vec<Place> = vec![];
            for y in 0..config::MAP_HEIGHT as i32 {
                field_row.push(Place::new(x, y));
            }
            field.push(field_row);
        }
        field
    }

    pub fn get_3x3_clone(&self, x: usize, y: usize) -> [[Place; 3]; 3] {
        [
            [
                self.field[x][y].clone(),
                self.field[x][y + 1].clone(),
                self.field[x][y + 2].clone(),
            ],
            [
                self.field[x + 1][y].clone(),
                self.field[x + 1][y + 1].clone(),
                self.field[x + 1][y + 2].clone(),
            ],
            [
                self.field[x + 2][y].clone(),
                self.field[x + 2][y + 1].clone(),
                self.field[x + 2][y + 2].clone(),
            ],
        ]
    }

    pub fn get_block_allignments(&mut self) -> Option<Vec<usize>> {
        let mut allignments: Vec<usize> = vec![];
        let mut blocks_count_per_row = [0; config::MAP_HEIGHT];

        for x in self.field.iter() {
            for y in x {
                if y.get_block_id() != None {
                    blocks_count_per_row[y.get_coord().y as usize] += 1;
                }
            }
        }

        for (i, count) in blocks_count_per_row.iter().enumerate() {
            if *count == config::MAP_LENGTH - 1 {
                allignments.push(i);
            }
        }

        if allignments.len() == 0 {
            None
        } else {
            Some(allignments)
        }
    }

    pub fn shift_map_down(&mut self, from_y: i32) {
        for x in 0..config::MAP_LENGTH {
            for y in 0..from_y {
                self.shift_place_down(x, (from_y - y) as usize);
            }
        }
        self.clear_first_row();
    }

    fn shift_place_down(&mut self, x: usize, y: usize) {
        let id = self.field[x][y - 1].get_block_id();
        let col = self.field[x][y - 1].get_color();

        self.field[x][y].set_block(id, col);
    }

    fn clear_first_row(&mut self) {
        for (x, row) in self.field.iter_mut().enumerate() {
            for (y, place) in row.iter_mut().enumerate() {
                if y == 0 {
                    *place = Place::new(x as i32, y as i32);
                } else {
                    break;
                }
            }
        }
    }

    pub fn print_map(&mut self) {
        self.win.clear(Color::BLACK);
        for row in self.field.iter() {
            for place in row.iter() {
                self.win.draw(place.get_rect_ref());
            }
        }
        self.win.display();
    }

    pub fn set_block(&mut self, x: usize, y: usize, block_id: usize, col: Color) {
        self.field[x][y].set_block(Some(block_id), col);
    }

    pub fn unset_block(&mut self, x: usize, y: usize) {
        self.field[x][y].unset_block();
    }
}
