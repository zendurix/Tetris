use crate::place::Coord;
use crate::place::Place;

pub fn rotate_block(mut area: [[Place; 3]; 3], block_id: usize) -> [Coord; 4] {
    let mut rotated_area = area.clone();

    let x_l = 2;
    let y_l = 2;
    for x in 0..=2 {
        for y in 0..=2 {
            if area[x][y].get_block_id() == Some(block_id) {
                println!("before x{} y{}", area[x][y].coord.x, area[x][y].coord.y);
            }
        }
    }

    for x in 0..=x_l {
        for y in 0..=y_l {
            rotated_area[x][y_l - y] = area[x_l - y][y_l - x].clone();
        }
    }
    area = rotated_area.clone();

    let mut coords: [Coord; 4] = [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(3, 0),
    ];
    let mut i = 0;
    for x in 0..=2 {
        for y in 0..=2 {
            if area[x][y].get_block_id() == Some(block_id) {
                coords[i] = Coord::new(
                    x as i32 + area[0][0].get_coord().x,
                    y as i32 + area[0][0].get_coord().y,
                ); //area[x][y].get_coord();

                println!("after  x{} y{}", area[x][y].coord.x, area[x][y].coord.y);
                i += 1;
            }
        }
    }

    for x in 0..=2 {
        for y in 0..=2 {
            if area[x][y].get_block_id() == Some(block_id) {
                println!("after2 x{} y{}", area[x][y].coord.x, area[x][y].coord.y);
            }
        }
    }
    coords
}

/*


let x_l = 2;
    let y_l = 2;
    for x in 0..=2 {
        for y in 0..=2 {
            if area[x][y].get_block_id() == Some(block_id) {
                println!("before x{} y{}", area[x][y].coord.x, area[x][y].coord.y);
            }
        }
    }

    for x in 0..=x_l {
        for y in 0..=y_l {
            rotated_area[x][y_l - y] = area[x_l - y][y_l - x].clone();
        }
    }

    let mut coords: [Coord; 4] = [
        Coord::new(0, 0),
        Coord::new(0, 0),
        Coord::new(0, 0),
        Coord::new(0, 0),
    ];
    let mut i = 0;
    for x in 0..=2 {
        for y in 0..=2 {
            if rotated_area[x][y].get_block_id() == Some(block_id) {
                coords[i] = rotated_area[x][y].get_coord();

                println!("after  x{} y{}", rotated_area[x][y].coord.x, rotated_area[x][y].coord.y);
                i += 1;
            }
        }
    }

    for x in 0..=2 {
        for y in 0..=2 {
            if rotated_area[x][y].get_block_id() == Some(block_id) {
                println!("after2 x{} y{}", rotated_area[x][y].coord.x, rotated_area[x][y].coord.y);
            }
        }
    }


    for x in 0..=2 {
        for y in 0..=2 {
            println!(
                "xx{} yy{}     x{} y{}",
                area[x][y].coord.x,
                area[x][y].coord.y,
                rotated_area[x][y].coord.x,
                rotated_area[x][y].coord.y
            );
        }
    }
    coords
}

/*

    for x in 0..=2 {
        for y in 0..=2 {
            println!(
                "xx{} yy{}     x{} y{}",
                area[x][y].coord.x,
                area[x][y].coord.y,
                rotated_area[x][y].coord.x,
                rotated_area[x][y].coord.y
            );
        }
    }

*/


*/
