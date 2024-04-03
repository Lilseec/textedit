use std::{io::{stdout, Write}, process::exit};

use crate::term::disable_raw_mode;

use super::Editor;

impl Editor {
    pub (super) fn command(&mut self) {
        let command = self.read();

        match command {
            'q' => self.quit(),
            _ => (),
        }
    }

    fn quit(&self) {
        disable_raw_mode();

        // clean the screen and move cursor to upper left corner
        print!("\x1b[2J");
        print!("\x1b[H");
        stdout().flush().expect("Flush failure");

        exit(1);
    }
}