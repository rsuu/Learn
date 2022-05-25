// USAGE:
// cargo run ./

use itertools::Itertools;
use procfs::process::Process;
use std::fs;
use std::io::{self, Write};
use std::iter;
use strsim::levenshtein;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    let path = fs::canonicalize(&args[1]).unwrap().display().to_string();
    let mut pep: Vec<(&str, usize)> = Vec::default();
    let mut mount_path: Vec<String> = Vec::default();

    for mount in Process::myself().unwrap().mountinfo().unwrap() {
        mount_path.push(mount.mount_point.display().to_string());
    }

    //sort_by_string_length(&mut mount_path, true);
    for p in mount_path.iter() {
        if path.starts_with(p.as_str()) {
            pep.push((p, levenshtein(p, &path)));
        }
    }
    pep = sort_by_n2(pep.into_iter(), false);

    io::stdout().write_all(pep[0].0.as_bytes())?;
    Ok(())
}

pub fn sort_by_string_length(v: &mut Vec<String>, big: bool) {
    if big {
        v.sort_by(|b, a| a.len().partial_cmp(&b.len()).unwrap());
    } else if !big {
        v.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    }
}
pub fn sort_by_n(v: &mut Vec<(&str, usize)>, big: bool) {
    if big {
        v.sort_by(|b, a| a.1.partial_cmp(&b.1).unwrap());
    } else if !big {
        v.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }
}
pub fn sort_by_n2<'s, I, T, N>(iter: I, big: bool) -> Vec<(T, N)>
where
    T: Into<&'s str>,
    N: Into<usize> + std::cmp::Ord,
    I: Iterator<Item = (T, N)>,
{
    if big {
        iter.sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
            .map(|(a, b)| (a, b))
            .collect_vec()
    } else if !big {
        iter.sorted_by(|b, a| Ord::cmp(&b.1, &a.1))
            .map(|(a, b)| (a, b))
            .collect_vec()
    } else {
        std::process::exit(1)
    }
}
