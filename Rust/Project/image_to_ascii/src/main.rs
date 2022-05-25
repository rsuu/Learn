use image::{self, DynamicImage};
use std::{path::PathBuf, str::from_utf8};

fn handle_img(imgs: &Imgs) -> DynamicImage {
    let img = image::open(&imgs.path).unwrap();

    let resized = imgs.size;

    img.resize_exact(resized.0, resized.1, image::imageops::FilterType::Nearest)
}

fn output_to_ascii(value: &u8) -> &str {
    let ascii_chars = [
        " ", ".", "^", ",", ":", "_", "=", "~", "+", "O", "o", "*", "#", "&", "%", "B", "@",
    ];

    let n_chars = ascii_chars.len() as u8;
    let step = 255_u8 / n_chars;
    for i in 1..(n_chars - 1) {
        let comp = step * i;
        if value < &comp {
            let idx = (i - 1) as usize;
            return ascii_chars[idx];
        }
    }

    ascii_chars[(n_chars - 1) as usize]
}

fn print_to_terminal(img: DynamicImage) {
    let imgbuf = img.to_luma8();
    let ascii_art = imgbuf
        .pixels()
        .map(|p| output_to_ascii(&p.0[0]))
        .fold(String::new(), |s, p| s + p);

    ascii_art
        .as_bytes()
        .chunks(imgbuf.width() as usize)
        .map(from_utf8)
        .for_each(|s| println!("{}", s.unwrap()));
}

fn main() -> Result<(), ()> {
    let args = std::env::args().collect::<Vec<_>>();

    let i;

    match &args.len() {
        2 => {
            i = Imgs {
                path: PathBuf::from(&args[1]),
                size: (80, 40),
            };
        }
        4 => {
            i = Imgs {
                path: PathBuf::from(&args[1]),
                size: (args[1].parse::<u32>().unwrap(), args[2].parse().unwrap()),
            };
        }
        _ => {
            std::process::exit(-1);
        }
    }

    let img = handle_img(&i);

    print_to_terminal(img);
    Ok(())
}

pub struct Imgs {
    pub path: PathBuf,
    pub size: (u32, u32),
}
