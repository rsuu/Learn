fn main() {
    let mut c = r#"(2+2)*1+(1+(1+1)*1)-(2+2)"#.chars().collect::<Vec<char>>();

    // 22+1∗111+1∗++22+−
    // 22+1
    // *
    // num -> num
    // ( -> s
    // + < ( -> s
    // + = + -> n
    // + < * * -> n
    // + > - - -> n
    // ) -> n
    // null -> s
    let mut pos = 0;
    let mut n = Vec::new(); // 数字
    let mut s = Vec::new(); // 操作符
                            // println!("{:#?}", c);

    for now in c.into_iter() {
        match f {
            '0'..='9' => {
                n.push(now);
            }

            '+' => {}
            '-' => {
                if let Some(last) = *s.last() {
                    match last {
                        '+' => {}
                        '-' => {}
                        '*' => {}
                        '/' => {}
                        '(' => {
                            s.push(now);
                        }
                        _ => panic!(),
                    }
                } else {
                    s.push(now);
                }
            }
            '*' => {
                if let Some(last) = *s.last() {
                    match last {
                        '+' => {}
                        '-' => {}
                        '*' => {}
                        '/' => {}
                        '(' => {
                            s.push(now);
                        }
                        _ => panic!(),
                    }
                } else {
                    s.push(now);
                }
            }
            '/' => {
                if let Some(last) = *s.last() {
                    match last {
                        '+' => {}
                        '-' => {}
                        '*' => {}
                        '/' => {}
                        '(' => {
                            s.push(now);
                        }
                        _ => panic!(),
                    }
                } else {
                    s.push(now);
                }
            }
            '(' => {
                s.push(now);
            }
            ')' => loop {
                if *s.last().expect("") == '(' {
                    s.pop();
                    break;
                } else {
                    n.push(*s.last().expect(""));
                    s.pop();
                }
            },

            _ => panic!(),
        }

        println!("{:?}", s);
    }

    println!("n: {:?}", n);

    let mut buf: u32 = 0;
    let mut v: Vec<u32> = Vec::new();
    let mut p = 0;

    while p < n.len() {
        match n[p] {
            '0'..='9' => {
                v.push(n[p].to_digit(10).unwrap());
            }

            '+' | '-' | '*' | '/' => {
                if v.len() >= 2 {
                    buf = expr(n[p], v[v.len() - 2], v[v.len() - 1]);
                    p += 1;
                }
            }

            _ => {
                println!("Err");
            }
        }
        p += 1;
    }

    println!("e: {}", (3 - 3) * 3 + (5 + (1 + 2) * 4) - (3 + 8));
    println!("v: {:?}", v);
    println!("buf: {:?}", buf);
}

fn expr(c: char, l: u32, r: u32) -> u32 {
    match c {
        '+' => l + r,
        '-' => l - r,
        '*' => l * r,
        '/' => l / r,

        _ => panic!(),
    }
}
// https://www.dcode.fr/reverse-polish-notation
