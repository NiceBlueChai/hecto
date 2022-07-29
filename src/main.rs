use crate::editor::Editor;

mod document;
mod editor;
mod row;
mod terminal;

pub use document::Document;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    println!("Hello, world!");
    Editor::default().run();
}
