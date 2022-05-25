#[derive(Debug)]
struct List {
    head: Option<Box<List>>,
    elem: &'static str,
}

impl List {
    fn new() -> List {
        List { head: None, elem: "" }
    }

    fn prepend(self, elem: &'static str) -> List {
        List {
            head: Some(Box::new(self)),
            elem: elem,
        }
    }

    fn len(&self) -> usize {
        match &self.head {
            Some(l) => l.len() + 1,
            None => 0,
        }
    }

    fn stringify(&self) -> String {
        match &self.head {
            Some(l) => format!("{}, {}", self.elem, l.stringify()),
            None => format!("NULL"),
        }
    }
}

fn main() {
    let mut list = List::new();
let s:Vec<_> = vec!["ss", "nn"];

for n in 0..s.len() {
    list = list.prepend(s[n]);
}

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
