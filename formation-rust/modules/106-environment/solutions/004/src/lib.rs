/// Gets whether the specified number is even (true), or odd (false)
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

/// Test that is_even returns true on even input
#[test]
fn is_true_when_even() {
    let value = is_even(2);
    println!("valeur de is_even: {}", value);
    assert!(value);
    assert_eq!(value, true);
}

/// Test that is_even returns false on odd input
#[test]
fn is_false_when_odd() {
    assert!(!is_even(1));
    assert_eq!(is_even(1), false);
}

#[cfg(test)]
mod tests {
    /// Gets whether the specified number is odd (true), or even (false)
    pub fn is_odd(num: i32) -> bool {
        num % 2 == 1
    }

    /// Test that is_odd returns false on even input
    #[test]
    fn is_false_when_even() {
        assert!(!is_odd(2));
        assert_eq!(is_odd(2), false);
    }

    /// Test that is_odd returns true on odd input
    #[test]
    fn is_true_when_odd() {
        assert!(is_odd(1));
        assert_eq!(is_odd(1), true);
    }
}

/// This function expects the input to be more than 10
pub fn must_be_more_than_ten(value: i32) {
    if value <= 10 {
        panic!("value should be more than 10, got {}", value);
    }
    println!("OK");
}

#[test]
fn must_be_more_than_ten_is_ok_with_11() {
    must_be_more_than_ten(11);
}

#[test]
#[should_panic]
fn must_be_more_than_ten_is_nok_with_10() {
    must_be_more_than_ten(10);
}

/// Safe division of a by b
pub fn safe_integer_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("b must not be 0"))
    } else {
        Ok(a / b)
    }
}

#[test]
fn safe_integer_divide_ok() -> Result<(), String> {
    safe_integer_divide(4, 2).map(|_| ())
}

#[test]
fn safe_integer_divide_nok() -> Result<(), String> {
    match safe_integer_divide(4, 0) {
        Ok(_) => Err(String::from("Expected an error")),
        Err(_) => Ok(())
    }
}
