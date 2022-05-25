fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    match args.len() {
        2 => {
            let v = get_freq(&args[1]);
            println!("{:?}", v);
        }
        3 => {
            let mut v = get_freq(&args[1]);
            let s = sort_by_freq(&mut v);
            println!("{:?}", s);
        }
        _ => {}
    }
}

fn get_freq(s: &str) -> Vec<(char, usize)> {
    let mut c: Vec<char> = s.chars().collect();

    c.sort_unstable();
    c.push(' ');

    // println!("{:#?}", c);

    let mut v: Vec<(char, usize)> = Vec::default();
    let mut n: usize = 1;

    for f in 1..c.len() {
        if c[f - 1] == c[f] {
            n += 1;
        } else {
            v.push((c[f - 1], n));
            n = 1;
        }
    }

    v
}

fn sort_by_freq(v: &mut [(char, usize)]) -> String {
    v.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", v);
    let mut u: Vec<char> = vec![];

    for j in v.iter() {
        for _ in 0..(j.1) {
            u.push(j.0);
        }
    }

    u.iter().collect::<String>()
}
