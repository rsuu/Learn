use std::{
    io::{self, BufWriter, Write},
    thread::sleep,
    time::Duration,
};

use bar::*;

fn progress1(w: bar::wh::UnixSize) {
    let lg: usize = w.0 as usize;
    let mut str0 = vec![' '; lg];

    // let mut str0: [char; lg] = [' '; lg];
    // https://en.wikipedia.org/wiki/Variable-length_array
    let mut bns = BufWriter::new(io::stdout());

    for (n, nu) in (1..=lg).enumerate() {
        bns.write_fmt(format_args!("\r{}", '[')).unwrap();
        str0[n] = '#';

        for i in str0.iter() {
            bns.write_fmt(format_args!("{}", i)).unwrap();
        }

        bns.write_fmt(format_args!("{}{}", ']', nu)).unwrap();

        bns.flush().unwrap();

        sleep(Duration::from_millis(40));
    }
}

fn main() {
    progress1(wh::terminal_size().unwrap());
}
