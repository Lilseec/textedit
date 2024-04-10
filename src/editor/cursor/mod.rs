use crate::term::{move_teminal_cursor_down, move_teminal_cursor_left, move_teminal_cursor_right, move_teminal_cursor_up};

#[derive(Default)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
    pub current_line_length: usize,
}

impl Cursor {
    pub fn move_right(&mut self) {
        if self.col == self.current_line_length {
            return;
        }

        let old_col = self.col;
        self.col = self.col+1;
        if !(self.col == old_col) { move_teminal_cursor_right() }
    }
    pub fn move_left(&mut self) {
        if self.col == 0 {
            return;
        }

        let old_col = self.col;
        self.col = self.col - 1;
        if !(self.col == old_col) { move_teminal_cursor_left() }
    }
    pub fn move_up(&mut self) {
        self.row = (self.row - 1).clamp(0, self.row);
        move_teminal_cursor_up()
    }
    pub fn move_down(&mut self) {
        self.row += 1;
        move_teminal_cursor_down()
    }
}