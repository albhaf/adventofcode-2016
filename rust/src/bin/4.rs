extern crate regex;

use std::io;
use std::collections::BTreeMap;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(.*)\[(.*)\]").unwrap();

    let mut sum: u32 = 0;
    let mut input = String::new();
    'line: loop {
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

                print!("{}", input);

                let c = re.captures(&input).unwrap();
                let mut name: Vec<&str> = c.at(1).unwrap().split("-").collect();
                let checksum = c.at(2).unwrap();

                let sector_id = name.pop().unwrap();

                let mut counts = BTreeMap::new();

                for n in name {
                    for c in n.chars() {
                        *counts.entry(c).or_insert(0) += 1;
                    }
                }

                for (ch, count) in &counts {
                    println!("{:?}: {}", ch, count);
                }

                println!("captures: {:?}", c);

                let mut check_count = 0;
                for c in checksum.chars() {
                    let count = match counts.get(&c) {
                        Some(c) => *c,
                        None => continue 'line,
                    };
                    println!("c: {}, count: {}, check_count: {}", c, count, check_count);
                    if check_count > 0 && count > check_count {
                        continue 'line;
                    }

                    check_count = count;

                    counts.remove(&c);
                }

                for (ch, count) in &counts {
                    println!("{:?}: {}", ch, count);
                }

                for (_, count) in &counts {
                    if *count > check_count {
                        continue 'line;
                    }
                }

                println!("adding");

                sum += sector_id.parse().unwrap();
            }
            Err(_) => panic!(""),
        }
    }

    println!("{}", sum);
}
