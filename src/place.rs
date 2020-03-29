use crate::config;

use sfml::graphics::{Color, RectangleShape, Shape, Transformable};
use sfml::system::Vector2f;

pub struct Coord {
    pub x: i32,
    pub y: i32,
}
impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

impl Clone for Coord {
    fn clone(&self) -> Coord {
        Coord { ..*self }
    }
}

impl std::cmp::PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Place {
    rect: RectangleShape<'static>,
    block_id: Option<usize>,
    pub coord: Coord,
    color: Color,
}

impl Place {
    pub fn new(x: i32, y: i32) -> Place {
        let mut rect = RectangleShape::new();
        rect.set_size(Vector2f::new(
            config::BLOCK_SIZE as f32,
            config::BLOCK_SIZE as f32,
        ));
        rect.set_fill_color(Color::BLACK);
        rect.set_position(Vector2f::new(
            (x * config::BLOCK_SIZE as i32) as f32,
            (y * config::BLOCK_SIZE as i32) as f32,
        ));
        Place {
            coord: Coord { x, y },
            color: Color::BLACK,
            block_id: None,
            rect,
        }
    }

    pub fn set_block(&mut self, id: Option<usize>, col: Color) {
        self.block_id = id;
        self.color = col;
        self.rect.set_fill_color(col);
    }

    pub fn unset_block(&mut self) {
        self.block_id = None;
        self.color = Color::BLACK;
        self.rect.set_fill_color(Color::BLACK);
    }
}

impl Clone for Place {
    fn clone(&self) -> Place {
        Place {
            rect: self.rect.clone(),
            coord: self.coord.clone(),
            block_id: self.block_id,
            color: self.color,
        }
    }
}

// getters / setters
impl Place {
    pub fn get_block_id(&self) -> Option<usize> {
        self.block_id
    }

    pub fn get_rect_ref(&self) -> &RectangleShape {
        &self.rect
    }

    pub fn get_coord(&self) -> Coord {
        self.coord.clone()
    }

    pub fn get_color(&mut self) -> Color {
        self.color
    }
}
