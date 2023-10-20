use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn clear_screen() {
        // print!("\x1b[2J");
        // \x1b = ESC
        // J command 
        // https://vt100.net/docs/vt100-ug/chapter3.html#ED
        // case the argument is 2, which says to clear the entire screen
        // \x1b[1J would clear the screen up to where the cursor is
        // \x1b[0J would clear the screen from the cursor up to the end of the screen, also the default argument
        print!("{}", termion::clear::All);
    }
    pub fn cursor_position(x: u16, y: u16) {
        // termion Goto method is also 1-based
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x, y));
    }
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine)
    }
}