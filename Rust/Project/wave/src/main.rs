use std::{thread, time};

fn main() {
    let mut s = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█']; // 0 - 7
    let speed = 24;
    let j = 7;
    let mut k = 0;

    print!("\x1B[2J\x1B[1;1H"); // clear screen

    loop {
        if k < s.len() {
            s[k] = s[j];
            k += 1;
        } else {
            k = 0;
        }

        print!("\x1B[H {: ^80}", String::from_iter(s));
        print!(
            "
               "
        );
        print!(
            "{: ^80}\n",
            String::from_iter(s.into_iter().rev().collect::<Vec<char>>())
        );
        thread::sleep(time::Duration::from_millis(speed));

        s = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    }
}
