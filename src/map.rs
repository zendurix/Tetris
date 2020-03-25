use crate::config;
use crate::place::Coord;
use crate::place::Place;

pub struct Map {
    pub field: Vec<Vec<Place>>, //  [[Place; config::MAP_HEIGHT]; config::MAP_LENGTH],
}

impl Map {
    pub fn new() -> Map {
        let field = Map::init_field();
        Map { field }
    }

    fn init_field() -> Vec<Vec<Place>> {
        let mut field: Vec<Vec<Place>> = vec![];
        for x in 0..config::MAP_LENGTH {
            let mut field_row: Vec<Place> = vec![];
            for y in 0..config::MAP_HEIGHT {
                field_row.push(Place::new(x as i8, y as i8));
            }
            field.push(field_row);
        }
        field
    }

    // pub fn set(&mut self, set: bool, x: usize, y: usize) {
    //    self.field[x][y] = set;
    // }

    //pub fn get(&self, x: usize, y: usize) -> char {
    //    if self.field[x][y] {
    //        '#'
    //    } else {
    //        ' '
    //     }
    //}

    pub fn print_map(&self) {
        for row in self.field.iter() {
            for place in row.iter() {
                print!("x y ");
            }
            print!("\n");
        }
    }
}
