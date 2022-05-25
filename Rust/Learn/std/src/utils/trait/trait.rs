trait Number {
    fn p(&self);
}

trait Numbers {
    fn s(&self);
}

impl Number for u8 {
    fn p(&self) {
        println!("{}", &self);
    }
}

impl Number for f64 {
    fn p(&self) {
        println!("{}", &self);
    }
}

impl Numbers for u8 {
    fn s(&self) {
        println!("{}", &self);
    }
}

impl Numbers for f64 {
    fn s(&self) {
        println!("{}", &self);
    }
}

fn test<T>(a: T)
where
    T: Number + Numbers,
{
    a.p();
    a.s();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        6_u8.p();
        7.0_f64.p();

        test(6);
        test(7.0);
        test(6_u8);
        test(7.0_f64);

        /* OUTPUT:
        7
        6
        6
        6
        7
        7
        6
        6
        6
        6
        */
        //test("a"); // 错误  因为我们没有为 &str 类型实现 Number 这个 trait 的功能
    }
}
