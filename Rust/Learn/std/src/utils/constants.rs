fn main() {
    const ONE: u8 = 1;
    static mut TWO: u8 = 2;

    let raw_p: *const u32 = &10;

    println!("{}", ONE);

    unsafe {
        println!("{}", TWO);

        two(&mut TWO);

        println!("{}", *raw_p);
    }
}

fn two(two: &mut u8) {
    *two = 3;
    println!("{}", two);
}
