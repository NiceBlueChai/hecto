use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::ClearType;
use crossterm::{execute, Result};
use std::io::stdout;

fn die(e: &std::io::Error) {
    let mut stdout = stdout();
    execute!(stdout, crossterm::terminal::Clear(ClearType::All)).unwrap();
    panic!("{}", e);
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
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
        let mut stdout = stdout();
        execute!(stdout, crossterm::terminal::Clear(ClearType::All))?;
        execute!(stdout, crossterm::cursor::MoveTo(0,0))?;
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_row();
            execute!(stdout, crossterm::cursor::MoveTo(0,0))?;
        }
        Ok(())
    }

    fn draw_row(&self) {
        for _ in 0..24 {
            println!("~\r");
        }
        println!("{:?}", crossterm::terminal::size().unwrap());
    }
    fn process_keypress(&mut self) -> Result<()> {
        let key_event = read_key()?;
        if let KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
        } = key_event
        {
            self.should_quit = true;
        }

        Ok(())
    }
}

fn read_key() -> Result<KeyEvent> {
    loop {
        if let Event::Key(event) = crossterm::event::read()? {
            return Ok(event);
        }
    }
}
