use crate::editor::Editor;

mod editor;
mod terminal;

pub use editor::Position;
pub use terminal::Terminal;

fn main() {
    println!("Hello, world!");
    Editor::default().run();
}
