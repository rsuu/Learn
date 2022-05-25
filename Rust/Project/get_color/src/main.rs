use ansi_term::Colour::RGB;
use std::{env, path::Path};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file_path = Path::new(&args[1]);

    if args.len() > 2 {
        if let Ok(mut img_b) = get_img_bytes(file_path) {
            let mode: &str = &args[2];
            match mode {
                "r" => {
                    let rand_num = get_rand_number(0, img_b.len());
                    get_rand_color(&img_b, rand_num);
                }
                "g" => {
                    get_color(&mut img_b);
                }
                _ => {
                    println!("cargo run -- 0.jpg g");
                    println!("OR");
                    println!("cargo run -- 0.jpg r");
                }
            }
        }
    } else {
        println!("cargo run -- 0.jpg g");
        println!("OR");
        println!("cargo run -- 0.jpg r");
    }
}

fn get_img_bytes(file: &Path) -> Result<Vec<u8>, ()> {
    if let Ok(im) = image::open(file) {
        Ok(im.into_bytes())
    } else {
        Err(())
    }
}
fn get_color(n: &mut Vec<u8>) {
    n.sort_unstable();
    n.dedup();
    if n.len() / 2 != 0 {
        for chunk in n[1..n.len()].chunks(3) {
            println!("{}", RGB(chunk[0], chunk[1], chunk[2]).paint("████"));
        }
    } else {
        for chunk in n.chunks(3) {
            println!("{}", RGB(chunk[0], chunk[1], chunk[2]).paint("████"));
        }
    }
}

fn get_rand_number(min: usize, max: usize) -> [usize; 3] {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();

    [
        rng.gen_range(min..=max),
        rng.gen_range(min..=max),
        rng.gen_range(min..=max),
    ]
}

fn get_rand_color(bytes: &[u8], num: [usize; 3]) {
    println!(
        "{}",
        RGB(bytes[num[0]], bytes[num[1]], bytes[num[2]]).paint("████")
    );
}
