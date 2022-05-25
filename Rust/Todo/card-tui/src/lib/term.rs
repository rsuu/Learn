use crate::card::Card;
use colorsys::Rgb;
use crossterm::{cursor, terminal, terminal::ClearType};
use serde::Deserialize;
use std::{collections::HashMap, fs};

use unicode_width::UnicodeWidthStr;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct TermColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TermColor {
    pub fn hex_to_rgb(color_code: &str) -> Self {
        let rgb = &Rgb::from_hex_str(color_code).unwrap();
        Self {
            r: Rgb::red(rgb) as u8,
            g: Rgb::green(rgb) as u8,
            b: Rgb::blue(rgb) as u8,
        }
    }

    pub fn color_from_code(color: Self) -> crossterm::style::Color {
        crossterm::style::Color::Rgb {
            r: color.r,
            g: color.g,
            b: color.g,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TermSize {
    pub w: u16,
    pub h: u16,
}

impl TermSize {
    pub fn new() -> Self {
        let size = terminal::size().unwrap();
        Self {
            w: size.0,
            h: size.1,
        }
    }
}

impl Default for TermSize {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let mut path = dirs_next::config_dir().expect("#00");
        path.push("card-tui");
        path.push("config.toml");
        let text: String = fs::read_to_string(path).expect("#01");
        let config: Config = toml::from_str(&text).unwrap();

        Config {
            keymap: config.keymap,
            theme: config.theme,
        }
    }

    pub fn ch(key: char, h: &HashMap<char, String>, card: &mut Card, change: &mut bool) {
        for (kmap, f) in h {
            if key == *kmap {
                match f.as_ref() {
                    // display a new card
                    "card" => {
                        if *change == false {
                            println!("{}", terminal::Clear(ClearType::All),);
                            print_fb_text(whsw(card.size.w, card.size.h, &card.text), 0, 0, 0);
                        } else {
                        }
                    }
                    // swap the front and back
                    // and you can't displat a new card after "turn"
                    // you have to use "next"
                    "turn" => {
                        *change = true;

                        println!("{}", terminal::Clear(ClearType::All),);
                        print_fb_text(whsw(card.size.w, card.size.h, &card.text), 0, 0, 0);
                        print_fb_text(whsw(card.size.w, card.size.h, &card.text), 0, 4, 1);
                    }
                    // next card
                    "next" => {
                        *change = false;
                        println!("{}", terminal::Clear(ClearType::All),);
                        print_fb_text(whsw(card.size.w, card.size.h, &card.text), 0, 0, 0);
                    }
                    "quit" => {}
                    _ => {}
                }
            }
        }
    }
}

//todo
pub fn whsw(width: u16, height: u16, text: &[String]) -> (u16, u16, Vec<String>) {
    let max_width = 40;
    let mut re: (u16, u16, Vec<String>) = (0, 0, Vec::new());

    for n in text.iter() {
        let text_width = n.width_cjk() as u16;

        if text_width >= max_width {
            let string = n.split("\\n").collect::<Vec<&str>>();
            let text_width = &format!("{:<width$}", "", width = max_width as usize);
            let join_text = "\r\n".to_owned() + text_width;
            re.0 = max_width;
            re.1 = height / 2;
            re.2.push(string.join(&join_text));
        } else {
            re.0 = (width - text_width) / 2;
            re.1 = height / 2;
            re.2.push(n.to_string());
        }
    }

    re
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub keymap: Keymap,
    pub theme: Theme,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Keymap {
    pub org: HashMap<char, String>,
    pub ctrl: HashMap<char, String>,
    pub alt: HashMap<char, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Theme {
    pub text: HashMap<char, String>,
}

pub fn print_fb_text(text: (u16, u16, Vec<String>), x: u16, y: u16, n: usize) {
    println!("{}{}", cursor::MoveTo(text.0 + x, text.1 + y), text.2[n],);
}
