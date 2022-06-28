# SSoS - 2. Lecture

Learning resources:

- [Rust book](https://doc.rust-lang.org/book/)
- [Exercism](https://exercism.org/tracks/rust)
- [Cheat](https://cheats.rs/#memory-lifetimes)

## **Rust**

Rust is a modern systems programming language focusing on safety, speed, and concurrency. It accomplishes these goals by being memory safe without using garbage collection.

Rust is a statically and strongly typed systems programming language. *statically*
means that all types are known at compile-time, *strongly*
means that these types are designed to make it harder to write incorrect programs.

The big difference from C and C++ is that Rust is *safe by default.* All memory accesses are checked. It is not possible to corrupt memory by accident.

## **Rust - Data Types**

Rust is a statically typed language. Every value in Rust is of a certain data type. The compiler can automatically infer data type of the variable based on the value assigned to it.

Use the **_let_** keyword to declare a variable.

## **Rust - Variables**

The data type is optional while declaring a variable in Rust. The data type is inferred from the value assigned to the variable.

## Mutable

Variables are immutable by default. Prefix the variable name with **mut** keyword to make it mutable. The value of a mutable variable can be changed.

## Shadowing

Rust allows programmers to declare variables with the same name. In such a case, the new variable overrides the previous variable.

## **Rust - Structure**

Arrays are used to represent a homogeneous collection of values. Similarly, a structure is another user defined data type available in Rust that allows us to combine data items of different types, including another structure. A structure defines data as a key-value pair.

The *struct* keyword is used to declare a structure. Since structures are statically typed, every field in the structure must be associated with a data type. The naming rules and conventions for a structure is like that of a variable. The structure block must end with a semicolon.

```rust
struct Person {
    first_name: String,
    last_name: String
}

fn main() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Smith".to_string()
    };
    println!("person {} {}", p.first_name,p.last_name);
}
```

## **Ownership**

Ownership is Rust’s most unique feature and has deep implications for the rest of the language.

It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works.

_Ownership_
 is a set of rules that governs how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no-longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory.

Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

### [Ownership Rules](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules)

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

- Each value in Rust has a variable that’s called its *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## **What is Borrowing?**

When a function transfers its control over a variable/value to another function temporarily, for a while, it is called borrowing. This is achieved by passing a reference to the variable **_(& var_name)_** rather than passing the variable/value itself to the function. The ownership of the variable/ value is transferred to the original owner of the variable after the function to which the control was passed completes execution.

### The Rules of References

- At any given time, you can have *either* one mutable reference *or* any number of immutable references.
- References must always be valid.

### In what order it will be droped?

```rust
type Id = u32;

struct Handle(Id);

// ~ destructor
impl Drop for Handle {
    fn drop(&mut self) {
        println!("handle {} dropped!", self.0)
    }
}

fn main() {
    let handle_0 = Handle(0);
    // TODO change handle_2 to _
    let handle_2 = create_handle();
    Handle(3);
}

fn create_handle() -> Handle {
    Handle(1);
    Handle(2)
}
```

## The Slice Type

A slice is a pointer to a block of memory. Slices can be used to access portions of data stored in contiguous memory blocks. It can be used with data structures like arrays, vectors and strings. Slices use index numbers to access portions of data. The size of a slice is determined at runtime.

*Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

Slices give you different *views* of the *same* array:

```rust
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];  // open range!

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
}
```

## Rust - Traits

Please note that Rust does not spell `struct` *class*. The keyword `class` in other languages is so overloaded with meaning that it effectively shuts down original thinking.

Let's put it like this: Rust structs cannot *inherit* from other structs; they are all unique types. There is no *sub-typing*. They are dumb data.

So how *does* one establish relationships between types? This is where *traits* come in.

```rust
trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);
}
```

## Rust - Enums

Enums are types which have a few definite values. For instance, a direction has only four possible values.

```rust
enum CarType {
   Hatch,
   Sedan,
   SUV
}
fn print_size(car:CarType) {
   match car {
      CarType::Hatch => {
         println!("Small sized car");
      },
      CarType::Sedan => {
         println!("Medium sized car");
      },
      CarType::SUV =>{
         println!("Large sized Sports Utility car");
      }
   }
}
fn main(){
   print_size(CarType::SUV);
   print_size(CarType::Hatch);
   print_size(CarType::Sedan);
}
```

## Rust - Option<T>

The `Option<T>` enum has two variants:

- `None`, to indicate failure or lack of value, and
- `Some(value)`, a tuple struct that wraps a `value` with type `T`.

Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not. Option types are very common in Rust code, as they have a number of uses:

```rust
fn main() {
    fn might_print(option: Option<&str>) {
        match option {
            Some(text) => println!("The argument contains the following value: '{}'", text),
            None => println!("The argument contains None."),
        }
    }
    let something: Option<&str> = Some("Some string is inside");
    let nothing: Option<&str> = None;
    might_print(something);
    might_print(nothing);
}
```

## Rust - Result

The return is annotated with the Result enum. We specify the types that the Result will contain when the function returns. If the string is long enough, we return a string literal. If there is an error, we will return a message that is a String. This explains the **`Result<&str, String>`**
.

```rust
fn main() {
    fn check_length(s: &str, min: usize) -> Result<&str, String> {
        if s.chars().count() >= min {
            return Ok(s);
        } else {
            return Err(format!("'{}' is not long enough!", s));
        }
    }

    let func_return = check_length("some string literal", 100);
    let a_str = match func_return {
        Ok(a_str) => a_str,
        Err(error) => panic!("Problem running 'check_length':\n {:?}", error),
    };
    println!("{}", a_str);
}
```

## What is a macro?

Rust provides a powerful macro system that allows meta-programming. As you have seen in the previous example, macros look like functions, except that their name ends with a bang(!), but instead of generating a function call, macros are expanded into source code that gets compiled with the rest of the program. Therefore, they provide more runtime features to a program unlike functions. Macros are an extended version of functions.
