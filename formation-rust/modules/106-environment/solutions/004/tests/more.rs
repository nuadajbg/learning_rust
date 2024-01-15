use exercice_unit_tests::is_even;

/// Test that is_even returns true on even input
#[test]
fn is_true_when_even() {
    assert!(is_even(2));
    assert_eq!(is_even(2), true);
}

/// Test that is_even returns false on odd input
#[test]
fn is_false_when_odd() {
    assert!(!is_even(1));
    assert_eq!(is_even(1), false);
}
