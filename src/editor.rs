use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

fn die(e: &std::io::Error) {
    panic!("{}", e);
}
#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _a = crossterm::terminal::enable_raw_mode();
        loop {
            let key = crossterm::event::read();
            match key {
                Ok(ev) => {
                    if let Event::Key(event) = ev {
                        match event {
                            KeyEvent {
                                code: KeyCode::Char('q'),
                                modifiers: KeyModifiers::CONTROL,
                            } => {
                                break;
                            }
                            // KeyEvent{code, modifiers} => println!("{:?}, {:?}", code, modifiers),
                            KeyEvent { code, modifiers } => {
                                if let KeyCode::Char(c) = code {
                                    if c.is_control() {
                                        println!("{:?}\r", c as u8);
                                    } else {
                                        println!("{:?} ({})\r", c as u8, c);
                                    }
                                } else {
                                    println!("{:?}", KeyEvent::new(code, modifiers));
                                }
                            }
                            // _ => println!("other ev");
                            // let c = b as char;
                            // if c.is_control() {
                            //     println!("{:?} \r", b);
                            //     // stdout().execute(MoveTo(0, 0)).unwrap();
                            // } else {
                            //     println!("{:?} ({})\r", b, c);
                            // }
                            // if b == to_ctrl_byte('q') {
                            //     break;
                            // }
                        }
                    }
                }
                Err(e) => die(&e),
            }
        }
    }
}
