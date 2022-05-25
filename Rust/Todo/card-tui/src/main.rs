use card::{card::Card, rand::*, term::*};

use std::io::stdout;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

trait Q {
    fn to_char(self) -> Result<char, ()>;
}

impl Q for KeyCode {
    fn to_char(self) -> Result<char, ()> {
        use crossterm::event::KeyCode::Char;
        match self {
            Char(n) => Ok(n),
            _ => Err(()),
        }
    }
}

fn main() -> Result<(), lexopt::Error> {
    if run().is_err() {}
    Ok(())
}

fn run() -> crossterm::Result<()> {
    let _term_fg_color = &TermColor::hex_to_rgb("#fff");
    let _term_bg_color = &TermColor::hex_to_rgb("#888");

    let mut stdout = stdout();
    let mut _sss: Vec<&str>;
    let mut state = false;
    let mut change = false;

    const HELP: &str = "help";

    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        EnterAlternateScreen,
        cursor::MoveTo(0, 0),
        cursor::Hide
    )?;

    keys(&mut state, &mut change);

    // clear output when exit
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn rng_n(filesmeta: &FilesMeta) -> usize {
    FilesMeta::rng_usize(0, filesmeta.lines)
}

pub fn new_text(filesmeta: &FilesMeta, n: usize) -> Vec<String> {
    FilesMeta::get_line(&filesmeta.filename, n)
        .split('\u{0009}')
        .map(str::to_string)
        .collect::<Vec<String>>()
}

fn keys(state: &mut bool, change: &mut bool) {
    let config = Config::default();
    let mut filesmeta = FilesMeta::default();
    filesmeta.lines = FilesMeta::get_line_num(&filesmeta.filename);
    let mut n: usize;
    let mut text: Vec<String>;
    let mut card: Card;
    n = rng_n(&filesmeta);
    text = new_text(&filesmeta, n);

    card = Card {
        text,
        size: TermSize::default(),
    };

    while !*state {
        if change == &mut false {
            n = rng_n(&filesmeta);
            text = new_text(&filesmeta, n);

            card = Card {
                text,
                size: TermSize::default(),
            };
        } else {
        }

        let event = read().unwrap();

        match event {
            Event::Key(KeyEvent {
                modifiers: KeyModifiers::NONE,
                code,
            }) => {
                if !*state {
                    Config::ch(
                        code.to_char().unwrap(),
                        &config.keymap.org,
                        &mut card,
                        change,
                    );
                } else {
                }
            }

            Event::Key(KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code,
            }) => {
                if code == KeyCode::Char('c') {
                    *state = true;
                }

                if !*state {
                    Config::ch(
                        code.to_char().unwrap(),
                        &config.keymap.org,
                        &mut card,
                        change,
                    );
                } else {
                }
            }

            Event::Key(KeyEvent {
                modifiers: KeyModifiers::SHIFT,
                code,
            }) => {
                if !*state {
                    Config::ch(
                        code.to_char().unwrap(),
                        &config.keymap.org,
                        &mut card,
                        change,
                    );
                } else {
                }
            }

            Event::Key(KeyEvent {
                modifiers: KeyModifiers::ALT,
                code,
            }) => {
                if !*state {
                    Config::ch(
                        code.to_char().unwrap(),
                        &config.keymap.org,
                        &mut card,
                        change,
                    );
                } else {
                }
            }

            // Match on multiple modifiers:
            Event::Key(KeyEvent { code, modifiers }) => {
                if modifiers == (KeyModifiers::ALT | KeyModifiers::SHIFT) {
                    if !*state {
                        Config::ch(
                            code.to_char().unwrap(),
                            &config.keymap.org,
                            &mut card,
                            change,
                        );
                    } else {
                    }
                }
            }

            _ => {}
        }
    }
}
