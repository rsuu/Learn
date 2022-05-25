use std::{
    env,
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = env::args().nth(1).unwrap();
    let files = File::open(file)?;

    let line: usize = env::args().nth(2).unwrap().parse::<usize>().unwrap();

    //println!("{:#?}",file.metadata().unwrap());
    let reader = BufReader::new(files);
    let p = reader.lines();

    for line in p.skip(line - 1) {
        match line {
            Ok(ref line) => {
                println!("{}", line);
            }
            _ => {}
        }

        break;
    }

    Ok(())
}



