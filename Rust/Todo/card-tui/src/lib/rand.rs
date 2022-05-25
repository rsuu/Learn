use crate::args;
use rand::Rng;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct FilesMeta {
    pub filename: String,
    pub lines: usize,
}

impl FilesMeta {
    pub fn check_file(filename: &str) -> File {
        File::open(filename).unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", filename, e))
    }

    pub fn rand_line(filename: &str, rand_num: usize) -> String {
        let f = BufReader::new(Self::check_file(filename));
        let lines = f.lines().map(|l| l.expect("Couldn't read line"));
        lines.skip(rand_num).into_iter().collect::<Vec<String>>()[0].clone()
    }

    pub fn get_line_num(filename: &str) -> usize {
        let f = BufReader::new(Self::check_file(filename));
        let lines = f.lines().map(|l| l.expect("Couldn't read line"));
        lines.count()
    }
    pub fn get_line(filename: &str, num: usize) -> String {
        Self::rand_line(filename, num)
    }

    pub fn rng_usize(start: usize, end: usize) -> usize {
        rand::thread_rng().gen_range(start..end)
    }

    pub fn new() -> Self {
        FilesMeta {
            filename: args::Args::default().thing,
            lines: 0,
        }
    }
}
impl Default for FilesMeta {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test_0() {}
}
