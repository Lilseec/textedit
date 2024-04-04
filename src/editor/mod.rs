use std::io::{stdout, Write};

use crate::term::{clear_screen, enable_raw_mode, get_terminal_size, press_backspace, press_enter, read, TermSize};

mod commands;

pub (crate) struct Editor {
    term_size: TermSize,
}

impl Editor {
    pub (crate) fn init() -> Self {
        let Some(term_size) = get_terminal_size() else {
            panic!("Couldn't get terminal window size");
        };

        clear_screen();

        enable_raw_mode();

        Self {
            term_size,
        }
    }

    pub (crate) fn start(&mut self) {
        loop {
            let input = read();

            match input.as_str() {
                ":" => self.command(),
                "\u{7f}" => press_backspace(),
                "\r" => press_enter(),
                _ => print!("{input}"),
            }
            stdout().flush().expect("Flush failure");
        }
    }
}