use crate::place::Coord;
use crate::place::Place;

pub fn rotate_block(mut area: [[Place; 3]; 3], block_id: usize) -> [Coord; 4] {
    let mut rotated_area = area.clone();
    let x_l = 2;
    let y_l = 2;

    for x in 0..=x_l {
        for y in 0..=y_l {
            rotated_area[x][y_l - y] = area[x_l - y][y_l - x].clone();
        }
    }
    area = rotated_area.clone();

    let mut coords: [Coord; 4] = [
        Coord::new(0, 0),
        Coord::new(0, 0),
        Coord::new(0, 0),
        Coord::new(0, 0),
    ];
    let mut i = 0;
    for x in 0..=2 {
        for y in 0..=2 {
            if area[x][y].get_block_id() == Some(block_id) {
                coords[i] = Coord::new(
                    x as i32 + area[0][0].get_coord().x,
                    y as i32 + area[0][0].get_coord().y - 1,
                );
                i += 1;
            }
        }
    }

    coords
}

pub fn rotate_i_block(coords: &[Coord; 4]) -> [Coord; 4] {
    let vertical = coords[0].x == coords[1].x;
    if vertical {
        let mut min_y = 100;
        let x = coords[0].x;
        for coord in coords {
            if coord.y < min_y {
                min_y = coord.y;
            }
        }
        return [
            Coord::new(x - 2, min_y + 2),
            Coord::new(x - 1, min_y + 2),
            Coord::new(x, min_y + 2),
            Coord::new(x + 1, min_y + 2),
        ];
    } else
    /* horizontal */
    {
        let mut min_x = 100;
        let y = coords[0].y;
        for coord in coords {
            if coord.x < min_x {
                min_x = coord.x;
            }
        }
        return [
            Coord::new(min_x + 2, y - 2),
            Coord::new(min_x + 2, y - 1),
            Coord::new(min_x + 2, y),
            Coord::new(min_x + 2, y + 1),
        ];
    }
}
