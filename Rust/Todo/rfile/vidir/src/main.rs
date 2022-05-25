// get ..
// read from tmpfile ..
// parse
// link
// rename
// delete

use std::{
    env, io,
    io::Write,
    path::{Path, PathBuf},
    process,
};
use tempfile::NamedTempFile;
use vidir::utils::{file, file::Ft, map, map::Txt, path};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        let list = get_list(&PathBuf::from(&args[1])).unwrap();
        let p = parse_list(list.0, list.1);
        check_list(p.0, p.1);
    } else {
        println!("cargo run -- ./");
        process::exit(-1);
    }
    // println!("{:#?}", hmap);
}

pub fn get_list(path: &Path) -> Result<(String, String), io::Error> {
    let p = path::get_path(path);

    let mut f = NamedTempFile::new()?;
    let editor: String = env::var("EDITOR").expect("export EDITOR");

    write_to_tmpfile(&mut f, &p)?;

    let fp: &Path = f.path();
    let bf = file::read_from_file(fp)?; // before

    process::Command::new(editor)
        .arg(fp)
        .status()
        .expect("Something went wrong");

    let af = file::read_from_file(fp)?; // after

    Ok((bf, af))
}

pub fn write_to_tmpfile(f: &mut NamedTempFile, text: &[String]) -> Result<(), io::Error> {
    for (n, t) in text.iter().enumerate().skip(1) {
        writeln!(f, "{}.\u{FFFD} {}", n, t)?;
    }

    Ok(())
}

pub fn parse_list(bf: String, af: String) -> (Vec<Txt>, Vec<Txt>) {
    let mut vbf: Vec<Txt> = Vec::default();
    let mut vaf: Vec<Txt> = Vec::default();

    for f in bf.lines() {
        vbf.push(map::ps(f).unwrap());
    }
    for v in af.lines() {
        vaf.push(map::ps(v).unwrap());
    }
    (vbf, vaf)
}
pub fn check_list(vbf: Vec<Txt>, vaf: Vec<Txt>) -> Result<(), fs_extra::error::Error> {
    if vbf.len() > vaf.len() {
        let mut k: Vec<bool> = Vec::default();
        let mut next = 0;

        for b in vbf.iter() {
            if b.0 == vaf[next].0 {
                k.push(true);
                if next + 1 < vaf.len() {
                    next += 1;
                }
            } else {
                k.push(false);
            }
        }

        for (n, s) in k.into_iter().enumerate() {
            if s {
                if vbf[n].1 != vaf[n].1 {
                    mv(vbf[n].1.as_ref().unwrap(), vaf[n].1.as_ref().unwrap())?;
                }
            } else {
                rm(vbf[n].1.as_ref().unwrap())?;
                // todo
            }
        }
        // println!("{:?}", k);
    } else if vbf.len() == vaf.len() {
        for b in &vbf {
            for a in &vaf {
                if b.0 == a.0 {
                    // println!("{} + {}", b.1.as_ref().unwrap(), a.1.as_ref().unwrap());
                    if b.1 != a.1 {
                        if mv(b.1.as_ref().unwrap(), a.1.as_ref().unwrap()).is_err() {
                            create(a.1.as_ref().unwrap())?;
                        }
                        // OR create_dir
                        // todo
                    }
                    break;
                }
            }
        }
    } else if vbf.len() < vaf.len() {
        process::exit(-2);
    }

    Ok(())
}

pub fn mv(a: &str, b: &str) -> Result<(), fs_extra::error::Error> {
    match file::get_type(a)? {
        Ft::S => {}
        Ft::F => {
            mv_file(a, b)?;
        }
        Ft::D => {
            mv_dir(a, b)?;
        }
    }

    Ok(())
}

pub fn mv_file(a: &str, b: &str) -> Result<(), fs_extra::error::Error> {
    use fs_extra::file::{move_file, CopyOptions};

    let options = CopyOptions::new();

    move_file(a, b, &options)?;

    Ok(())
}
pub fn mv_dir(a: &str, b: &str) -> Result<(), fs_extra::error::Error> {
    use fs_extra::dir::{move_dir, CopyOptions};

    let options = CopyOptions::new();

    if get_exists(a) {
        move_dir(a, b, &options)?;
    } else {
        create_dir(a)?;
        // create and move files
        // todo
    }

    Ok(())
}

pub fn rm(path: &str) -> Result<(), fs_extra::error::Error> {
    match file::get_type(path)? {
        Ft::S => {}
        Ft::F => {
            rm_file(path)?;
        }
        Ft::D => {
            rm_dir(path)?;
        }
    }

    Ok(())
}
pub fn rm_file(file: &str) -> Result<(), fs_extra::error::Error> {
    use fs_extra::file::remove;
    remove(file)?;
    Ok(())
}
pub fn rm_dir(dir: &str) -> Result<(), fs_extra::error::Error> {
    use fs_extra::dir::remove;
    remove(dir)?;
    Ok(())
}

pub fn create(dir: &str) -> Result<(), fs_extra::error::Error> {
    create_dir(dir)?;
    Ok(())
}
pub fn create_dir(dir: &str) -> Result<(), fs_extra::error::Error> {
    use fs_extra::dir::create;
    create(dir, false)?;
    Ok(())
}

pub fn get_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
    // rerutn Ft
    // todo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mv() {
        mv("tests/a.txt", "tests/b.txt").unwrap_or(());
    }
}
