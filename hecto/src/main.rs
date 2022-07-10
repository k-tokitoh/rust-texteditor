use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn ctrl_to_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({}) \r", b, c);
        }
        if b == ctrl_to_byte('q') {
            break;
        }
    }
}
