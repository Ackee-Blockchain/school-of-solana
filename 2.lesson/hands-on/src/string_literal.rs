#[allow(dead_code)]
pub fn print_string_literal() {
    let course: &str = "Summer School of Solana";
    let lecture: &str = "Rust";
    let company: &'static str = "AckeeBlockchain";

    println!(
        "I do attend {} lecture on {} from {}",
        course, lecture, company
    );
}
