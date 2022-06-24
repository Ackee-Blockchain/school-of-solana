pub fn check_length(s: &str, min_length: usize) -> Result<&str, String> {
    if s.len() < min_length {
        return Err(format!("String is too short: {}", s));
    }
    Ok(s)
}

pub fn main() {
    let s = "Hello, world!";
    let result = match check_length(s, 200) {
        Ok(s) => s,
        Err(e) => panic!("Problem running 'check_length': {}", e),
    };
}