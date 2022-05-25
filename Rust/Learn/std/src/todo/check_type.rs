use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
fn main() {
    let x = 21;
    println!("{}", type_of(x));
}
