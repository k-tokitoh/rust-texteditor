use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Ctrl('q') => break,
                Key::Char(c) => {
                    println!("{:?} ({}) \r", c as u8, c);
                }
                _ => println!("{:?}\r", key),
            },
            Err(e) => die(e),
        }
    }
}
