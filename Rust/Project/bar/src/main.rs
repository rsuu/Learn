use std::fmt::{Debug, Display};
use std::{thread, time};

struct Kkp {
    a: char,
    b: char,
}

fn some_func<T: Debug + Display>(f: T) -> String {
    let q: String = format!("{}", f);
    q
}

fn main() {
    let mut a: Vec<char>;
    let mut tmp: Vec<char> = vec![];

    let ms = time::Duration::from_millis(24);
    let kkp = Kkp { a: '=', b: '>' };

    for _f in 0..=100 {
        tmp.push(kkp.a);
        a = tmp.clone();
        println!(
            "\x1B[2J[{}{}]",
            some_func(a.iter().collect::<String>()),
            kkp.b
        );
        //print!("\x1B[2J\x1B[1;1H");
        thread::sleep(ms);
    }
}
