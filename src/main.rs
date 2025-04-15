use std::io; // user input
use rand::

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    // declare mutable variable
    // String::new() creates a new instance of a string
    // :: -> "new" is an associated function of the String type
    // String::new() creates a new empty string
    // let mut guess = String::new() -> create a new variable bound to a new empty instance of a string
    let mut guess = String::new();

    // std::io::stdin returns an instance of std::io::Stdin which is a type that represents a handle to the std input for terminal
    io::stdin() // allows us to handle user input
        .read_line(&mut guess) // calls read_line method on std input handle and store it in guess
        // read_line appends by default instead of overwriting.
        // & syntax indicates that mut guess is a reference (no copy) ~~ pointer?
        // references are immutable by default. Hence the &mut.
        .expect("Failed to read line"); // read_line() appends to the passed string
        // but it also returns a Result value. 
        // Result is an enumeration which is a type that can be in one of multiple possible state
        // each possible state is called a variant.
        // Result's varaints are Ok & Err. 
        // Result is an instance of a type so it has methods. One of these is expect.
        // expect will crash program and display the arg error msg if read_line() returns Err.

    println!("You guessed: {guess}");
    // println!("You guessed: {}", guess); // alternative syntax
}
