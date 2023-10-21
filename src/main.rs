#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use terminal::Terminal;
pub use row::Row;
pub use editor::Position;
// re-exporting the Terminal struct at the top level and make it reachable via crate::Terminal

fn main() {
    Editor::default().run();
}
