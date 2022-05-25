// TODO
// save to file
// read from file

use std::{
    collections::BTreeMap,
    io::{self, BufRead, Write},
};

fn main() {
    let t_stin = io::stdin();
    let mut stin = t_stin.lock();
    let mut done = false;
    let mut nums: usize = 0;
    let mut data: BTreeMap<usize, String> = BTreeMap::new();

    while !done {
        let mut input = String::new();
        write!(&mut io::stdout(), "> ");
        io::stdout().flush();
        stin.read_line(&mut input);

        if input.is_empty() {
            println!("\nLeaving, bye!");
            break;
        }

        if (&input as &str).ends_with('\n') {
            input.pop();
        }

        done = text(&mut nums, input, &mut data);
    }
}

pub fn ls(data: &BTreeMap<usize, String>) {
    for (nums, text) in data {
        println!("{}    \t   {}", nums, text);
    }
}

pub fn remove(data: &mut BTreeMap<usize, String>, nums: usize) {
    data.remove(&nums);
}

pub fn add(data: &mut BTreeMap<usize, String>, nums: &usize, text: String) {
    data.insert(*nums, text);
}

pub fn text(nums: &mut usize, text: String, data: &mut BTreeMap<usize, String>) -> bool {
    let mut m = false;
    let input = text.trim().split(' ').collect::<Vec<_>>();

    match input[0] {
        ":h" => {
            print!("{:?}", input);
        }

        ":l" => {
            ls(data);
        }

        ":q" => {
            m = true;
        }

        ":d" => {
            remove(data, input[1].parse::<usize>().unwrap());
        }

        ":a" => {
            add(data, nums, input[1].to_string());
            *nums += 1;
        }

        _ => {}
    }
    m
}
