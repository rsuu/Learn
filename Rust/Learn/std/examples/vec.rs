struct Foo<T> {
    vector: Vec<T>,
}

impl<T> Foo<T> {
    fn at(&mut self, i: usize) -> Option<&T> {
        if let Some(x) = self.vector.get(i) {
            return Some(x);
        }
        self.at(i)
    }
}

fn swap_array<T: Copy + Sized>(array: &mut [T], slice: &[T], start: usize) {
    if (start + slice.len()) <= array.len() {
        let mut n = 0;
        for f in start..=slice.len() + 1 {
            array[f] = slice[n];
            n += 1;
        }
    } else {
        eprintln!("1");
    }
}

fn main() {
    let mut a = [7, 8, 9, 10, 11];
    let slice = [1, 2];

    swap_array::<u8>(&mut a, &slice, 2);

    println!("{:#?}", a);
}
