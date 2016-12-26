use std::io;

fn main() {
    let buttons: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let mut button: (i32, i32) = (1, 1);

    let mut code = Vec::new();

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.len() == 0 {
                    break;
                }
                for s in input.chars() {
                    let (mut y, mut x) = button;
                    match s {
                        'U' => y -= 1,
                        'L' => x -= 1,
                        'D' => y += 1,
                        'R' => x += 1,
                        '\n' => break,
                        _ => panic!(s.to_string()),
                    }
                    if y < 0 {
                        y = 0;
                    }
                    if y > 2 {
                        y = 2;
                    }
                    if x < 0 {
                        x = 0;
                    }
                    if x > 2 {
                        x = 2;
                    }
                    button = (y, x);
                }
                let (y, x) = button;
                code.push(buttons[y as usize][x as usize]);
            }
            Err(_) => panic!(""),
        }

        input.clear();
    }

    println!("{:?}", code);

}
