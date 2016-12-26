use std::io;

fn main() {
    let mut valid = 0;

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.len() == 0 {
                    break;
                }
                let s = input.trim();
                let sides: Vec<&str> = s.split_whitespace().collect();
                let a: u32 = sides[0].parse().unwrap();
                let b: u32 = sides[1].parse().unwrap();
                let c: u32 = sides[2].parse().unwrap();

                if a + b > c && b + c > a && a + c > b {
                    valid += 1;
                }
            }
            Err(_) => panic!(""),
        }

        input.clear();
    }

    println!("valid: {}", valid);
}
