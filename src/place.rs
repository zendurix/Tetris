pub struct Coord {
    pub x: i8,
    pub y: i8,
}

impl Clone for Coord {
    fn clone(&self) -> Coord {
        Coord { ..*self }
    }
}

pub struct Place {
    have_block: bool,
    pub block_id: Option<i8>,
    pub coord: Coord,
}

impl Place {
    pub fn new(x: i8, y: i8) -> Place {
        Place {
            coord: Coord { x, y },
            have_block: false,
            block_id: None,
        }
    }
}

// place doesn't hold any complicated data, so it is safe to copy
impl Clone for Place {
    fn clone(&self) -> Place {
        Place {
            coord: self.coord.clone(),
            ..*self
        }
    }
}
