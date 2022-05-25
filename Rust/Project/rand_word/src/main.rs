use rand::Rng;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Args {
    pub thing: String,
}

pub struct FilesMeta<'fname> {
    pub filename: &'fname str,
    pub lines: usize,
}

fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;
    let mut filesmeta = FilesMeta {
        filename: &args.thing,
        lines: 0,
    };

    filesmeta.lines = get_line_num(filesmeta.filename);

    let mut n: usize;

    for _ in 0..=10 {
        n = rng_usize(0, filesmeta.lines);

        println!("{}", get_line(filesmeta.filename, n).trim_start());
    }
    Ok(())
}

fn check_file(filename: &str) -> File {
    File::open(filename).unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", filename, e))
}

fn rand_line(filename: &str, rand_num: usize) -> String {
    let f = BufReader::new(check_file(filename));
    let lines = f.lines().map(|l| l.expect("Couldn't read line"));
    lines.skip(rand_num).into_iter().collect::<Vec<String>>()[0].clone()
}

fn get_line_num(filename: &str) -> usize {
    let f = BufReader::new(check_file(filename));
    let lines = f.lines().map(|l| l.expect("Couldn't read line"));
    lines.count()
}
fn get_line(filename: &str, num: usize) -> String {
    rand_line(filename, num)
}

fn rng_usize(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut thing = None;
    let _number = 0;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) => {
                thing = Some(val.into_string()?);
            }
            Long("help") => {
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        thing: thing.ok_or("missing argument THING")?,
    })
}

#[cfg(test)]
mod test {

    #[test]
    fn test_0() {}
}
