use std::io;
use std::io::Read;

enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut dir = Direction::North;

    let sin = io::stdin();
    let mut buf = String::new();

    match sin.lock().read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("Failed reading stdin"),
    };

    for s in buf.split(", ") {
        let (first, second) = s.split_at(1);
        dir = match first {
            "R" => {
                match dir {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                    Direction::East => Direction::South,
                }
            }
            "L" => {
                match dir {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                    Direction::East => Direction::North,
                }
            }
            &_ => panic!("Unrecognized direction"),
        };

        let count: i32 = match second.trim().parse() {
            Ok(c) => c,
            Err(e) => panic!(second.to_string()),
        };

        match dir {
            Direction::North => y += count,
            Direction::South => y -= count,
            Direction::West => x -= count,
            Direction::East => x += count,
        }
    }

    println!("x: {}, y: {}", x, y);
}
