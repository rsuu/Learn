fn main() {
    println!("Hello, world!");
}
fn p() {
    use std::iter::Peekable;

    let mut res = Vec::new();
    let mut a = "1 +1".chars().peekable();

    while let Some(&n) = a.peek() {
        match n {
            '0'..='9' => {
                res.push("n");
                a.next();
            }
            '+' => {
                res.push("add");
                a.next();
            }
            ' ' => {
                res.push(" ");
                a.next();
            }
            _ => {
                res.push(" ");
                a.next();
            }
        }
    }
}
