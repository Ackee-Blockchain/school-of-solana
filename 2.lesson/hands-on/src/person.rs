struct Person {
    first_name: String,
    last_name: String,
}

impl Person{
    fn new_person(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
}

pub fn main() {
    let person: Person = Person::new_person("John", "Doe");

    println!("Person: {} {}", person.first_name, person.last_name);
}