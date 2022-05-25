use std::fmt;

struct A<'a, 'b> {
    a: usize,
    b: &'a str,
    c: &'a str,
    db: B<'b>,
}

struct B<'b> {
    d: &'b str,
}

impl<'a, 'b> fmt::Display for A<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

impl<'a, 'b> A<'a, 'b> {
    fn o(&self) -> usize {
        self.a
    }

    fn d(&self) -> &Self {
        self
    }

    fn p(&self) {
        println!("{}", self);
    }
}

fn main() {
    let b = B { d: "ddd" };
    let a = A {
        a: 0,
        b: "bbb",
        c: "ccc",
        db: b,
    };

    a.o();
    a.p();
}
