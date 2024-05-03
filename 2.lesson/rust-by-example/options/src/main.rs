/// Prints a message based on the provided `Option<&str>`.
///
/// This function takes an optional string reference. If the option contains
/// a value, it prints the value in a formatted string that announces the content.
/// If the option is `None`, it simply states that no value is present.
pub fn might_print(option: Option<&str>) {
    match option {
        Some(text) => println!("The argument contains the following value: '{}'", text),
        None => println!("The argument contains None."),
    }
}

fn main() {
    // Demonstrating function usage with a string
    might_print(Some("Solana is great"));

    // Demonstrating function usage with none
    might_print(None);
}
