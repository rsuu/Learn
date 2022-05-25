use colorsys::{Hsl, Rgb};
use owo_colors::{colors::*, BgColorDisplay, FgColorDisplay, OwoColorize};
use regex::Regex;
use std::borrow::Cow;
use std::io::{self, Write};

fn main() {
    argf_help_set_color();
}

struct ArgsHelp<'a> {
    usage: &'a str,
    flags: &'a str,
    subcommands: &'a str,
    args: &'a str,

    f_version: (&'a str, &'a str, &'a str, &'a str),
    f_help: (&'a str, &'a str, &'a str, &'a str),
    f_c: (&'a str, &'a str, &'a str, &'a str),

    s_help: (&'a str, &'a str, &'a str, &'a str),
}

fn argf_help_set_color() {
    let a = ArgsHelp {
        usage: "USAGE:",
        flags: "FLAGS:",
        subcommands: "SUBCOMMANDS",
        args: "",

        f_version: ("-v ,", "--version", "", "Prints version information"),
        f_help: ("-h ,", "--help", "", "Prints help information"),
        f_c: (
            "-c ,",
            "--cc",
            "<file>",
            "awdawdawd
            awdawdawdawdawd
            awdawdaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),

        s_help: ("help", "", "", "Prints help information"),
    };

    let rgb0 = &LocalRgb::from("#98971a");
    let rgb1 = &LocalRgb::from("#d79921");

    println!(
        "{usage}

{flags}


    {f_c}
    {f_help}

    {f_version}

{subcommand}
    {s_help}",
        usage = a.usage.truecolor(rgb1.r, rgb1.g, rgb1.b),
        flags = a.flags.truecolor(rgb1.r, rgb1.g, rgb1.b),
        subcommand = a.subcommands.truecolor(rgb1.r, rgb1.g, rgb1.b),
        f_help = argf_help_set_color_once(rgb0, a.f_help),
        f_version = argf_help_set_color_once(rgb0, a.f_version),
        s_help = argf_help_set_color_once(rgb0, a.s_help),
        f_c = argf_help_set_color_once(rgb0, a.f_c),
    );
}
fn argf_help_set_color_once<'a>(rgb: &LocalRgb, s: (&'a str, &'a str, &'a str, &'a str)) -> String {
    format!(
        "{:<1} {:<1} {:<1}
            {:<1}",
        s.0.truecolor(rgb.r, rgb.g, rgb.b),
        s.1.truecolor(rgb.r, rgb.g, rgb.b),
        s.2.truecolor(rgb.r, rgb.g, rgb.b),
        s.3.truecolor(rgb.r, rgb.g, rgb.b),
    )
}

/*

s 0.2.1
ssss

USAGE:
    s [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
*/

#[derive(Debug)]
struct LocalRgb {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<&colorsys::Rgb> for LocalRgb {
    // &colorsys::Rgb <-> LocalRgb
    fn from(rgb: &colorsys::Rgb) -> Self {
        Self {
            r: Rgb::red(rgb) as u8,
            g: Rgb::green(rgb) as u8,
            b: Rgb::blue(rgb) as u8,
            a: 0,
        }
    }
}

impl From<&str> for LocalRgb {
    // &str <-> LocalRgb
    fn from(hex: &str) -> Self {
        LocalRgb::from(&Rgb::from_hex_str(hex).unwrap())
    }
}

impl From<(u8, u8, u8)> for LocalRgb {
    // (u8, u8, u8) <-> LocalRgb
    fn from(rgb: (u8, u8, u8)) -> Self {
        Self {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
            a: 0,
        }
    }
}
impl From<(u8, u8, u8, u8)> for LocalRgb {
    // (u8, u8, u8, u8) <-> LocalRgb
    fn from(rgb: (u8, u8, u8, u8)) -> Self {
        Self {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
            a: rgb.3,
        }
    }
}
