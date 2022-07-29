use crate::Terminal;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::Result;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Editor {
    pub fn run(&mut self) {
        let _a = crossterm::terminal::enable_raw_mode();
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
        Terminal::move_to(&self.cursor_position);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_row();
            Terminal::move_to(&self.cursor_position)
        }
        Terminal::cursor_show();
        Terminal::flush();
        Ok(())
    }

    fn draw_row(&self) {
        let height = self.terminal.size().height as usize;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
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
        let size = self.terminal.size();
        let height = size.height as usize;
        let width = size.width as usize;
        match kev.code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                if x < width {
                    x = x.saturating_add(1)
                }
            }
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height,
            _ => (),
        }
        self.cursor_position = Position { x, y };
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
