use crate::Document;
use crate::Row;
use crate::Terminal;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::Result;
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Default for Editor {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let fnm = &args[1];
            Document::open(&fnm).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: Default::default(),
            terminal: Default::default(),
            cursor_position: Default::default(),
            offset: Default::default(),
            document,
        }
    }
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<()> {
        Terminal::cursor_hide();
        //NOTE: 防止第一行从光标处打印，先将光标移动到左上角
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position)
        }
        Terminal::cursor_show();
        Terminal::flush();
        Ok(())
    }

    fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height as usize;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("hecto version -- {}", VERSION);
        let size = self.terminal.size();
        let width = size.width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_add(1));
        welcome_message = format!("~{}{}", spaces, &welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", &welcome_message);
    }
    fn move_cursor(&mut self, kev: KeyEvent) {
        let Position { mut x, mut y } = self.cursor_position;
        let Position { x: ox, y: mut oy } = self.offset;
        let size = self.terminal.size();
        let height = size.height as usize;
        let width = size.width as usize;
        match kev.code {
            KeyCode::Up => {
                if y > 0 {
                    y = y.saturating_sub(1);
                } else {
                    oy = oy.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                } else {
                    oy = oy.saturating_add(1);
                }
            }
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height,
            _ => (),
        }
        self.cursor_position = Position { x, y };
        self.offset = Position { x: ox, y: oy };
    }

    fn process_keypress(&mut self) -> Result<()> {
        let key_event = self.terminal.read_key()?;

        match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            } => self.should_quit = true,
            KeyEvent { code, modifiers: _ } => match code {
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::PageUp
                | KeyCode::PageDown => self.move_cursor(key_event),
                _ => (),
            },
        }

        Ok(())
    }
}
