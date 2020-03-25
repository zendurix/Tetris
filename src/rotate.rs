use crate::block::Block;
use crate::place::Coord;
use crate::place::Place;

use std::iter::Iterator;

pub fn rotate_block(area: &[[Place; 3]; 3]) {
    //[Coord; 4] {

    let mut rotated_area = area.clone();

    let x_l = 2;
    let y_l = 2;

    for x in 0..=x_l {
        for y in 0..=y_l {
            //println!{"xx{} yy{}   x{} y{}", xx, yy, x, y};

            // rotated_area[x_l - y][y_l - x] = area[x][y_l - y].clone();

            rotated_area[x][y_l - y] = area[x_l - y][y_l - x].clone();

            // println!(
            ////     "xx{} yy{}   x{} y{}",
            //     area[x_l - y][y_l - x].coord.x,
            //     area[x_l - y][y_l - x].coord.y,
            //      rotated_area[x][y_l - y].coord.y
            // );

            // println!{"AA xx{} yy{}   x{} y{}", x_v[x_l-y], y_v[y_l-x], x_v[x], y_v[y_l-y]};
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
}
