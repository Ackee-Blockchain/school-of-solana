pub fn contains_char(s: &str, c: char) -> Option<&str> {
    if s.chars().any(|x| x == c) {
        return Some(s);
    } else {
        return None;
    }
}

pub fn main() {
    let text = "Hello, world!";
    let char1 = 'o';
    let char2 = 'z';
    let result1 = contains_char(text, char1);
    let result2 = contains_char(text, char2);
    println!("{:?}, {:?}", result1, result2);
}