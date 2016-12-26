
use std::io;
use std::collections::BTreeMap;

fn main() {
    let mut columns = Vec::new();
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

                let mut i = 0;
                for c in input.trim().chars() {
                    match columns.get(i) {
                        None => columns.insert(i, BTreeMap::new()),
                        _ => (),
                    }

                    let columns: &mut BTreeMap<char, i32> = match columns.get_mut(i) {
                        Some(c) => c,
                        None => unreachable!(),
                    };
                    *columns.entry(c).or_insert(0) += 1;
                    i += 1;
                }
            }
            Err(_) => panic!(""),
        }
    }

    for column in columns {
        let mut highest = 0;
        let mut character = '\0';
        for (ch, count) in column {
            if count > highest {
                highest = count;
                character = ch;
            }
        }

        print!("{}", character);
    }
    println!("");

}
