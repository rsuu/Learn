fn main() {}

#[cfg(test)]
mod test {
    #[test]
    fn test_ceil() {
        // Returns the smallest integer greater than or equal to a number.

        let a = 3.01_f64;
        let b = 4.0_f64;
        let c = -3.54_f64;

        assert_eq!(a.ceil(), 4.0);
        assert_eq!(b.ceil(), 4.0);
        assert_eq!(c.ceil(), -3.0);
    }

    #[test]
    fn test_round() {
        // Returns the nearest integer to a number. Round half-way cases away from 0.0.

        let f = 3.3_f64;
        let g = -3.3_f64;

        assert_eq!(f.round(), 3.0);
        assert_eq!(g.round(), -3.0);
    }

    #[test]
    fn test_fract() {
        // Returns the fractional part of a number.

        let x = 3.6_f64;
        let y = -3.6_f64;
        let abs_difference_x = (x.fract() - 0.6).abs();
        let abs_difference_y = (y.fract() - (-0.6)).abs();

        assert!(abs_difference_x < 1e-10);
        assert!(abs_difference_y < 1e-10);
    }
}
