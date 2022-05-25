fn main() {
    struct A {
        x: i32,
        y: i32,
    }
    let a = A { x: 7, y: 9 };

    drop(a.x); // Move out of one field of the struct

    println!("{}", a.y); // Ok: Still use another field of the struct

    let c = || println!("{}", a.y); // Error: Tries to capture all of `a`
    c();
}
