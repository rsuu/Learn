use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    let arg_file = &args[1];

    if let Ok(lines) = read_file(arg_file) {
        let mut n: usize = 0;
        let arg_number = args[2].parse::<usize>().unwrap();

        if (arg_number == 0) {
            for line in lines.flatten() {
                n += 1;
                println!("{}  {}", n, line);
            }
        } else {
            for line in lines.flatten() {
                n = n + 1;
                if n == arg_number {
                    println!("{}  {}", n, line);
                    break;
                } else {
                }
            }
        }
    }
}

fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
