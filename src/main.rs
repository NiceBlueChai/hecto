use crate::editor::Editor;

mod editor;



fn main() {
    println!("Hello, world!");
    let editor = Editor::default();
    editor.run();
}
