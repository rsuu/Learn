use std::env;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!("{}", get_password(args[1].parse::<usize>().unwrap()));
}

fn get_password(ag_num: usize) -> String {
    use rand::{thread_rng, Rng};

    let all: Vec<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '~', '!', '@', '#', '$', '%', '^', '&', '*', '(',
        ')', '_', '+', '-', '+', '`', '[', ']', '{', '}', '"', ',', '|', '.', '<', '>', '/', '?',
    ];

    let all_len: usize = all.len();
    let n: usize = ag_num;

    let mut rng = thread_rng();
    let mut q: Vec<char> = vec![];
    let password: String;

    for _ in 0..n {
        q.push(all[rng.gen_range(0..all_len)]);
    }

    password = q.into_iter().collect::<String>();
    password
}
