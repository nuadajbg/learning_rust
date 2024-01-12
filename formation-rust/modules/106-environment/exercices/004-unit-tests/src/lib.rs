/// Gets whether the specified number is even (true), or odd (false)
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

/// This function expects the input to be more than 10
pub fn must_be_more_than_ten(value: i32) {
    if value <= 10 {
        panic!("value should be more than 10, got {}", value);
    }
    println!("OK");
}

/// Safe division of a by b
pub fn safe_integer_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("b must not be 0"))
    } else {
        Ok(a / b)
    }
}
