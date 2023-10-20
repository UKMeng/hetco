#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;

use editor::Editor;
pub use terminal::Terminal;
// re-exporting the Terminal struct at the top level and make it reachable via crate::Terminal

fn main() {
    Editor::default().run();
}
