use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    pub fn run(&self) {
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
                Err(e) => die(&e),
            }
        }
    }
}

fn die(e: &io::Error) {
    panic!("{}", e);
}
