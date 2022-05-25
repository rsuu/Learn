pub trait FBIter: Iterator {
    fn prev(&mut self) -> Option<Self::Item>;
}

pub struct VecIter<'a, Item>
where
    Item: 'a,
{
    index: Option<usize>,
    vector: &'a Vec<Item>,
}

impl<'a, Item> VecIter<'a, Item> {
    fn new(vector: &'a Vec<Item>) -> VecIter<'a, Item> {
        VecIter {
            index: None,
            vector: vector,
        }
    }
}

impl<'a, Item> Iterator for VecIter<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<&'a Item> {
        let index = match self.index {
            Some(i) => i + 1,
            None => 0,
        };

        self.index = Some(index);
        self.vector.get(index)
    }
}

impl<'a, Item> FBIter for VecIter<'a, Item> {
    fn prev(&mut self) -> Option<&'a Item> {
        let index = match self.index {
            Some(0) | None => return None,
            Some(i) => i - 1,
        };

        self.index = Some(index);
        self.vector.get(index)
    }
}

fn main() {
    let v = vec![0, 1, 2, 3, 4, 5];
    let mut iterator = VecIter::new(&v);

    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.prev());
    println!("{:?}", iterator.prev());
}
