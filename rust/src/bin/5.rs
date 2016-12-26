extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io;

fn main() {
    let mut hasher = Md5::new();

    let mut input = String::new();
    let input = match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim(),
        Err(_) => panic!(""),
    };

    let mut index = 0;
    for _ in 0..8 {
        for i in index..std::u64::MAX {
            hasher.reset();
            hasher.input(input.as_bytes());
            hasher.input(i.to_string().as_bytes());

            let mut output = [0; 16];
            hasher.result(&mut output);

            let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
            if first_five == 0 {
                for &byte in output.iter() {
                    print!("{:X}", byte);
                }
                println!("");
                let sixth = output[2] & 0xff;
                println!("i: {}, sixth: {:x}", i, sixth);
                index = i + 1;
                break;
            }
        }
    }
}
