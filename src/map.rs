use crate::factories::{spawn, spawn_decor};
use crate::world::World;

use std::fs::File;
use std::io::Read;

use tcod::console::{Console, Root};

pub fn generate_map(t: &Root, w: &mut World) -> Vec<Vec<char>> {
    //fill with #s
    let mut map = vec![vec!['#'; t.height() as usize]; t.width() as usize];
    for x in 5..t.width() as usize {
        for y in 5..t.height() as usize {
            map[x][y] = ' '
        }
    }
    //gap
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '#' {
                spawn(w, "wall", x as i32, y as i32);
            }
        }
    }

    map
}

pub fn generate_map_alt(t: &Root, w: &mut World) {
    let mut file = File::open("assets/test.txt").unwrap();

    let mut temp_str = String::new();

    file.read_to_string(&mut temp_str).unwrap();

    temp_str = temp_str.replace('\u{00A0}', " ");

    for (y, line) in temp_str.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != ' ' {
                let x_i32 = x as i32;
                let y_i32 = y as i32;
                if ch != '#' {
                    spawn_decor(w, ch, x_i32, y_i32);
                } else {
                    spawn(w, "wall", x_i32, y_i32);
                }
            }
        }
    }
}
