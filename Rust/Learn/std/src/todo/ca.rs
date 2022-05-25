pub fn division(a: usize, b: usize) -> usize {
    let mut n: usize;

    if b * b < a {
        n = b;
    } else if b * b == a {
        n = b;
    } else {
        n = 0;
    }

    while b * n < a {
        n += 1;
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division() {
        assert_eq!(division(6, 3), 2);
        assert_eq!(division(4, 4), 1);
        assert_eq!(division(81, 9), 9);
        // assert_eq!(division(3, 2), 1.5); // exit
    }
}
