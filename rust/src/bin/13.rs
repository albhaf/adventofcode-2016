
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let sin = io::stdin();
    let mut input = String::new();

    match sin.lock().read_to_string(&mut input) {
        Ok(_) => (),
        Err(_) => panic!("Failed reading stdin"),
    };

    let puzzle_input: u32 = input.trim().parse().unwrap();

    let start = (1 as u32, 1 as u32);
    let goal = (31 as u32, 39 as u32);

    let solution = {
        let mut queue: Vec<(HashSet<(u32, u32)>, (u32, u32))> = Vec::new();
        let mut visited = HashSet::new();
        let mut solution: Option<HashSet<(u32, u32)>> = None;

        queue.push((HashSet::new(), start));

        loop {
            if queue.len() == 0 {
                break;
            }

            let (path, current) = queue.remove(0);

            if current == goal {
                solution = Some(path);
                break;
            }

            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            let (x, y) = current;

            if !is_wall(puzzle_input, x - 1, y) {
                let pos = (x - 1, y);
                let mut path = path.clone();
                path.insert(pos);
                queue.push((path, pos));
            }
            if !is_wall(puzzle_input, x, y - 1) {
                let pos = (x, y - 1);
                let mut path = path.clone();
                path.insert(pos);
                queue.push((path, pos));
            }
            if !is_wall(puzzle_input, x + 1, y) {
                let pos = (x + 1, y);
                let mut path = path.clone();
                path.insert(pos);
                queue.push((path, pos));
            }
            if !is_wall(puzzle_input, x, y + 1) {
                let pos = (x, y + 1);
                let mut path = path.clone();
                path.insert(pos);
                queue.push((path, pos));
            }
        }

        if solution == None {
            panic!("No solution!");
        }

        solution.unwrap()
    };

    for y in 0..55 {
        for x in 0..45 {
            match (x, y) {
                (x, y) if (x, y) == start => print!(" O "),
                (x, y) if (x, y) == goal => print!(" X "),
                (x, y) if solution.contains(&(x, y)) => print!(" O "),
                (x, y) if is_wall(puzzle_input, x, y) => print!(" # "),
                (_, _) => print!(" . "),
            }
        }

        println!("");
    }

    println!("solution length: {}", solution.len());
}

fn is_wall(input: u32, x: u32, y: u32) -> bool {
    let first = (x * x) + (3 * x) + (2 * x * y) + y + (y * y);
    let second = first + input;
    let third = second.count_ones();
    (third % 2) != 0
}
