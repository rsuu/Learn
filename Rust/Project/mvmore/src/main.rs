use std::fs::{metadata, rename};

fn main() {
    //todo
    // mv p/1 -> p/2

    let args: Vec<_> = std::env::args().collect();
    mmm(&args[1]);
}

pub fn mmm(input: &str) {
    if (input.len() / 2 == 0) {
        panic!("E01: error");
    } else {
        let ps: Vec<&str> = input.split(',').collect();

        for g in ps.chunks(2) {
            if metadata(g[0]).is_ok() || g[1].is_empty() {
                std::process::exit(1);
            } else {
                rename(g[0], g[1]).unwrap(); // rename
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_mmm() {
        let a = "1,2,3,4";
        mmm(a);
        // 1 -> 2
        // 3 -> 4
    }
}
