
use std::collections::HashMap;
use std::io;

fn main() {

    let mut registers = HashMap::new();
    registers.insert('a', 0 as u32);
    registers.insert('b', 0 as u32);
    registers.insert('c', 0 as u32);
    registers.insert('d', 0 as u32);

    let instructions: Vec<String> = {
        let mut vec = Vec::new();
        let mut input = String::new();

        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if input.len() == 0 {
                        break;
                    }
                    vec.push(input.trim().to_string());
                    input.clear();
                }
                Err(_) => panic!(""),
            }
        }

        vec
    };

    let mut pc = 0;
    loop {
        println!("pc: {}", pc);
        println!("registers: {:?}, registers", registers);

        let line = match instructions.get(pc as usize) {
            Some(l) => l,
            None => break,
        };

        println!("instruction: {}", line);

        let mut split = line.split_whitespace();
        let op = split.next().unwrap();
        match op {
            "cpy" => {
                let source = split.next();
                let dest = split.next().unwrap().chars().next().unwrap();
                let value: u32 = match source {
                    Some(x) if x == "a" || x == "b" || x == "c" || x == "d" => {
                        *registers.get(&x.chars().next().unwrap()).unwrap()
                    }
                    Some(x) => x.parse().unwrap(),
                    None => panic!("Incomplete instruction: {}", line),
                };
                registers.insert(dest, value);
            }
            "inc" => {
                let reg = split.next().unwrap().chars().next().unwrap();
                *registers.get_mut(&reg).unwrap() += 1;
            }
            "dec" => {
                let reg = split.next().unwrap().chars().next().unwrap();
                *registers.get_mut(&reg).unwrap() -= 1;
            }
            "jnz" => {
                let source = split.next();
                let value: u32 = match source {
                    Some(x) if x == "a" || x == "b" || x == "c" || x == "d" => {
                        *registers.get(&x.chars().next().unwrap()).unwrap()
                    }
                    Some(x) => x.parse().unwrap(),
                    None => panic!("Incomplete instruction: {}", line),
                };
                if value != 0 {
                    let y = split.next().unwrap();
                    pc += match y.chars().next() {
                        Some('-') => {
                            let i: i32 = (&y[1..]).parse().unwrap();
                            -i
                        }
                        Some(_) => y.parse().unwrap(),
                        None => panic!(""),
                    };
                    continue;
                }
            }
            _ => panic!("Unrecognized instruction: {}", line),
        }

        pc += 1;
    }

    println!("{:?}", registers);
}
