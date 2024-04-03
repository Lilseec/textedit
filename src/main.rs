use editor::Editor;

mod term;
mod editor;

fn main() {
    let mut editor = Editor::init();
    editor.start();
}