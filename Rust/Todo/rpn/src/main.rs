use std::io::prelude::*;

fn main() {
    let ins = std::io::stdin();
    for s in ins.lock().lines() {
        rpn(s.unwrap());
    }
}

pub fn rpn(s: String) {
    let mut stk: Vec<f64> = Vec::default();
    let mut err = false;

    for tk in s.split_whitespace() {
        if let Ok(x) = tk.parse() {
            stk.push(x);
        } else {
            err = stk.len() < 2;
            if err {
                break;
            }
            let (y, x) = (stk.pop().unwrap(), stk.pop().unwrap());
            match tk {
                "+" => stk.push(x + y),
                "-" => stk.push(x - y),
                "*" => stk.push(x * y),
                "/" => stk.push(x / y),
                _ => {
                    err = true;
                    break;
                }
            }
        }
    }
    if !err && stk.len() == 1 {
        println!("{}", stk[0]);
    } else if err || stk.len() > 1 {
        println!("error");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // assert_eq!(2, rpn(String::from("1 1 +")));
    }

    #[test]
    fn test_add_add() {
        // assert_eq!(3, rpn(String::from("1 1 1 + +")));
    }
}
