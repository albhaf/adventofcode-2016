
use std::io;

fn main() {
    let mut input = String::new();

    let mut count = 0;

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

                if has_abba(&input) {
                    count += 1;
                }

            }
            Err(_) => panic!(""),
        }
    }

    println!("{}", count);
}

fn has_abba(s: &str) -> bool {
    let mut a1 = '\0';
    let mut b1 = '\0';
    let mut b2 = '\0';

    let mut abba = false;
    let mut in_brackets = false;

    for a2 in s.chars() {
        if a2 == '[' || a2 == ']' {
            a1 = '\0';
            b1 = '\0';
            b2 = '\0';

            in_brackets = !in_brackets;
        }

        if a1 == a2 && b1 == b2 && a1 != b1 {
            if in_brackets {
                return false;
            } else {
                abba = true;
            }
        }

        a1 = b1;
        b1 = b2;
        b2 = a2;
    }

    abba
}
