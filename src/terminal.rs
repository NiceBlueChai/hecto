use crate::Position;
use crossterm::event::Event;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Write};

pub struct Terminal {
    size: Size,
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Default for Terminal {
    fn default() -> Self {
        crossterm::terminal::enable_raw_mode().expect("Failed enable raw mode");
        let size = crossterm::terminal::size().expect("Failed get terminal size");
        Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        }
    }
}

impl std::ops::Drop for Terminal {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("Failed disable raw mode");
    }
}

impl Terminal {
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn cursor_hide() {
        execute!(stdout(), crossterm::cursor::Hide).unwrap();
    }
    pub fn flush() {
        stdout().flush().unwrap();
    }
    pub fn cursor_show() {
        execute!(stdout(), crossterm::cursor::Show).unwrap();
    }
    pub fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }
    pub fn clear_current_line() {
        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
    }

    pub fn cursor_position(pos: &Position) {
        let Position { x, y } = pos;
        let x = *x as u16;
        let y = *y as u16;
        execute!(stdout(), crossterm::cursor::MoveTo(x, y)).unwrap();
    }

    pub fn read_key(&mut self) -> Result<crossterm::event::KeyEvent, std::io::Error> {
        loop {
            let ev = crossterm::event::read()?;
            match ev {
                Event::Key(event) => return Ok(event),
                Event::Resize(w, h) => {
                    self.size = Size {
                        width: w,
                        height: h,
                    };
                    continue;
                }
                _ => continue,
            }
        }
    }
}
