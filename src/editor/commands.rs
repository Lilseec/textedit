use std::{io::{stdout, Write}, process::exit};

use crate::term::{clear_screen, disable_raw_mode, read};

use super::Editor;

impl Editor {
    pub (super) fn command(&mut self) {
        print!(":");
        stdout().flush().expect("Flush failure");

        let command = read();

        match command.as_str() {
            "q" => self.quit(),
            _ => (),
        }
    }

    fn quit(&self) {
        disable_raw_mode();

        clear_screen();

        exit(1);
    }
}