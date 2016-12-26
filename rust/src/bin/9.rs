
use std::io;
use std::io::Read;

#[derive(Clone, Copy)]
enum State {
    None,
    MarkerLength,
    MarkerCount,
    Mark,
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    let stdin = io::stdin();
    match stdin.lock().read_to_string(&mut input) {
        Ok(_) => (),
        Err(_) => panic!("Failed reading stdin"),
    };

    let mut state = State::None;

    let mut marker_length = 0;
    let mut marker_length_buf = String::new();
    let mut marker_count = 0;
    let mut marker_count_buf = String::new();
    let mut marker_buf = String::new();

    for c in input.trim().chars() {
        match (c, state) {
            ('x', State::MarkerLength) => {
                marker_length = marker_length_buf.parse().unwrap();
                marker_length_buf.clear();
                state = State::MarkerCount;
            }
            (')', State::MarkerCount) => {
                marker_count = marker_count_buf.parse().unwrap();
                marker_count_buf.clear();
                state = State::Mark;
            } 
            (_, State::Mark) => {
                marker_buf.push(c);
                if marker_buf.len() >= marker_length {
                    for _ in 0..marker_count {
                        output.push_str(&marker_buf);
                    }
                    marker_buf.clear();
                    state = State::None;
                    continue;
                }
            }
            (_, State::MarkerLength) => {
                marker_length_buf.push(c);
            }
            (_, State::MarkerCount) => {
                marker_count_buf.push(c);
            }

            ('(', _) => state = State::MarkerLength,
            (_, _) => output.push(c),
        }
    }

    println!("{}", output);
    println!("{}", output.len());
}
