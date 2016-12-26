
use std::io;

fn main() {
    let mut screen = [[false; 50]; 6];

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.len() == 0 {
                    break;
                }
                let input = {
                    let s = input.to_string();
                    input.clear();
                    s
                };

                let mut row = input.split_whitespace();
                match row.next() {
                    Some("rect") => {
                        let size: Vec<&str> = row.next().unwrap().split("x").collect();
                        let width: u32 = size[0].parse().unwrap();
                        let height: u32 = size[1].parse().unwrap();

                        for y in 0..height {
                            for x in 0..width {
                                screen[y as usize][x as usize] = true
                            }
                        }
                    }
                    Some("rotate") => {
                        let row_cmd = row.next();
                        let cmd: Vec<&str> = row.next()
                                                .unwrap()
                                                .split("=")
                                                .collect();
                        let _ = row.next();
                        let shift: usize = row.next().unwrap().parse().unwrap();

                        let value: usize = cmd.get(1)
                                              .unwrap()
                                              .parse()
                                              .unwrap();
                        match row_cmd {
                            Some("row") => {
                                let row = screen[value];
                                let mut new = [false; 50];
                                let mut i: usize = 0;
                                for b in row.iter() {
                                    let mut j: usize = i + shift;
                                    j = (j % 50);
                                    new[j] = *b;
                                    i += 1;
                                }
                                screen[value] = new;
                            }
                            Some("column") => {
                                let mut new = [false; 6];
                                for i in 0..6 {
                                    let mut j: usize = i + shift;
                                    j = (j % 6);
                                    new[j] = screen[i][value];
                                }
                                for i in 0..6 {
                                    screen[i][value] = new[i];
                                }
                            }
                            _ => panic!("Unrecognized command: {}", input),
                        }
                    }
                    _ => panic!("Unrecognized command: {}", input),
                }
            }
            Err(_) => panic!(""),
        }
    }

    let mut count = 0;
    for a in screen.iter() {
        for b in a.iter() {
            if *b {
                count += 1;
                print!(" 1 ");
            } else {
                print!(" 0 ");
            }
        }
        println!("\n");
    }

    println!("{}", count);
}
