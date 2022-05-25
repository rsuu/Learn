use colorsys::{Hsl, Rgb};
use std::io::{self, Write};
use termcolor::{
    Buffer, BufferWriter, Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock,
    WriteColor,
};

// "#fff"
// (0.0.0)
fn main() {
    write_green().unwrap();
}

fn write_green() -> io::Result<()> {
    let mut bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    buffer.fg("aaa", "#fff")?;
    buffer.flush()?;
    buffer.bg("aaa\n\n", (1, 1, 1))?;
    buffer.flush()?;
    buffer.fg("aaa\n\n", "#fff")?;
    buffer.flush()?;
    bufwtr.print(&buffer)?;

    Ok(())
}

trait SetColor {
    fn fg<'a, T>(&mut self, text: &str, color: T) -> io::Result<()>
    where
        T: Into<LocalRgb>;
    fn bg<'a, T>(&mut self, text: &str, color: T) -> io::Result<()>
    where
        T: Into<LocalRgb>;
}

impl SetColor for Buffer {
    fn fg<T>(&mut self, text: &str, color: T) -> io::Result<()>
    where
        T: Into<LocalRgb>,
    {
        self.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::from(&color.into()))))?;

        write!(self, "{}", text)?;

        Ok(())
    }
    fn bg<T>(&mut self, text: &str, color: T) -> io::Result<()>
    where
        T: Into<LocalRgb>,
    {
        self.set_color(ColorSpec::new().set_bg(Some(termcolor::Color::from(&color.into()))))?;

        write!(self, "{}", text)?;

        Ok(())
    }
}

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

impl From<&LocalRgb> for termcolor::Color {
    // &LocalRgb <-> termcolor::Color
    fn from(rgb: &LocalRgb) -> Self {
        termcolor::Color::Rgb(rgb.r, rgb.g, rgb.b)
    }
}
