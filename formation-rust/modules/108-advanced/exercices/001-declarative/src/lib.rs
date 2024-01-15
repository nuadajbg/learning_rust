use std::f64;

/// The tolerance to use
pub const EPSILON: f64 = 0.01_f64;

// absolute value of f64
// ```
// let x = -42.0_f64;
// let y = x.abs();
// ```

#[test]
fn tests() {
    assert_eq!(eqt!(32.0_f64, 32.001_f64), true);
    assert_eq!(eqt!(32.001_f64, 32.0_f64), true);
    assert_eq!(eqt!(-32.0_f64, -32.001_f64), true);
    assert_eq!(eqt!(-32.001_f64, -32.0_f64), true);
    assert_eq!(eqt!(32.0_f64, 32.1_f64), false);
    assert_eq!(eqt!(32.1_f64, 32.0_f64), false);
    assert_eq!(eqt!(-32.0_f64, -32.1_f64), false);
    assert_eq!(eqt!(-32.1_f64, -32.0_f64), false);
}
