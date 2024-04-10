use std::io::{stdout, Write};

use crate::term::{clear_screen, enable_raw_mode, erase_line_from_cursor, get_terminal_size, press_backspace, press_enter, read, TermSize};

use self::cursor::Cursor;

mod commands;
mod cursor;

pub (crate) struct Editor {
    term_size: TermSize,
    file: Vec<String>,
    cursor: Cursor,
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
            file: Vec::new(),
            cursor: Cursor::default(),
        }
    }

    pub (crate) fn start(&mut self) {
        loop {
            let input = read();

            match input.as_str() {
                ":" => self.command(),
                "\u{7f}" => press_backspace(),
                "\r" => press_enter(),
                "\u{1b}[C" => self.cursor.move_right(),
                "\u{1b}[D" => self.cursor.move_left(),
                _ => self.print(input),
            }
            
            stdout().flush().expect("Flush failure");
        }
    }

    fn print(&mut self, input: String) {
        let col = self.cursor.col;
        let (line, actual_start_idx) = self.insert_str_in_current_line(input, col);

        // clears the line in terminal from cursor's position and inserts updated line
        erase_line_from_cursor();
        print!("{}", &line[actual_start_idx..]);

        let new_length = line.chars().count();
        let delta_length = new_length - self.cursor.current_line_length;
        self.cursor.current_line_length = new_length;
        
        // self.cursor.col + 1 because terminal cursor origin is insane (1,1) and my cursor origin is sane (0,0)
        self.cursor.col += delta_length;
        print!("\x1b[1;{}H", self.cursor.col + 1);
        
    }

    fn insert_str_in_current_line(&mut self, string: String, mut idx: usize) -> (&String, usize) {
        if self.cursor.row > self.file.len() { panic!("Row index out of file bounds") }
        if self.cursor.row == self.file.len() { self.file.push(String::new()) }

        let line = &mut self.file[self.cursor.row];
        let mut  actual_start_idx: usize = 0;

        // idx + 1 is needed because actual_start_idx is unsigned and starts with 0 (this always makes first loop iteration decrease idx by 1, making it useless)
        // actual_start_idx -= 1 fixes off-by-one error (could be also done with commented out if statement, but branchless ftw)
        idx += 1;
        while idx > 0 {
            if line.is_char_boundary(actual_start_idx) {
                idx -= 1;
                // if idx == 0 { break; }
            }
            actual_start_idx += 1;
        }
        actual_start_idx -= 1;

        line.insert_str(actual_start_idx, &string);
        (line, actual_start_idx)
    }
}