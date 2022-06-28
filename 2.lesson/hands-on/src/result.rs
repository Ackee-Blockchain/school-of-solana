pub fn check_length(s: &str, min: usize) -> Result<&str, String> {
    if s.chars().count() >= min {
        return Ok(s);
    } else {
        return Err(format!("'{}' is not long enough!", s));
    }
}

// let a = check_length("some str", 5);
// let b = check_length("another str", 300);
// dbg!(a); // Ok("some str",)
// dbg!(b); // Err("'another str' is not long enough!",)
