use std::io::{stdin, stdout, Bytes, Read, Stdin, Write};

use crate::term::{enable_raw_mode, get_terminal_size, TermSize};

mod commands;

pub (crate) struct Editor {
    stdin_iter: Bytes<Stdin>,
    term_size: TermSize,
}

impl Editor {
    pub (crate) fn init() -> Self {
        let Some(term_size) = get_terminal_size() else {
            panic!("Couldn't get terminal window size");
        };
        let stdin_iter = stdin().bytes();

        // clean the screen and move cursor to upper left corner
        print!("\x1b[2J");
        print!("\x1b[H");
        stdout().flush().expect("Flush failure");

        enable_raw_mode();

        Self {
            term_size,
            stdin_iter,
        }
    }

    pub (crate) fn start(&mut self) {
        loop {
            let input = self.read();
            print!("{}", input);
            stdout().flush().expect("Flush failure");

            match input {
                ':' => self.command(),
                _ => (),
            }
        }
    }

    fn read(&mut self) -> char {
        loop {
            let input = self.stdin_iter
                .next()
                .and_then(|res| res.ok());

            match input {
                Some(char) => return char as char,
                None => continue,
            }
        }
    }
}