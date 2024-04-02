mod term;

use term::get_terminal_size;

#[derive(Debug)]
pub struct TermSize {
    pub rows: u16,
    pub cols: u16,
}

fn main() {
    let Some(term_size) = get_terminal_size() else {
        panic!("Couldn't get terminal window size");
    };
    
    println!("{:?}", term_size)
}
