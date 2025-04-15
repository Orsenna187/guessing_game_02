use std::io; // user input
use rand::Rng; // Rng is a 'trait', not sure what that is yet.
use std::cmp::Ordering; 

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    // rand::tread_rng() -> random number generator
    // method of the Rng trait that we brought into scope with 'use'
    // gen_range -> method of the random number generator.
    // rust infers type of secret_number from later comparison with explicitly type guess var!!!
    // u8 = unsigned 8-bit integer: 0-255
    // look at this space efficiency
    let secret_number = rand::rng()
        .random_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop {
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
    
        // We're shadowing the guess variable. 
        // trim() removes whitespace. Required because satisfying read_line() requires pressing enter, which adds a newline.
        // parse() converts string to the type of our var. Error handling important here.
        // parse() returns a Result type. 
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num, // using match for more complete error handling. Shows that Result type is an Enum.
            Err(_) => {
                println!("That's not a number!");
                continue;
            }, // _ is a catch-all value. i.e: match all Err values.
        };
    
        println!("You guessed: {guess}");
        // println!("You guessed: {}", guess); // alternative syntax
    
        // use to bring a type called std::cmp::Ordering into scope from standard library.
        // the Ordering type is an enum with variants [Less, Greater, Equal]
        // cmp method compare two value, can be called on anything comparable.
        // compare guess to &secret_number and returns a variant of Ordering enum.
        // match is made of arms. Arm = pattern to match against and the code to run if the value given to match fits that pattern.
    
        match guess.cmp(&secret_number) { // cmp returns a variant of the type Ordering which is an enum
            // Enum::Variant => statement to run if true.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}

// Learnings
// - everything is a type. String::new() instead of creating an empty string.
// - not forcing to declare a variable as a string, but it needs to be initialized as an empty string.
// - Strong static type system. But with SMART type inference. Nice!
// - soft-forced error handling
// - &mut var -> references are pointers with less footguns. Zero-copy.
// - :: syntax for fcts associated with traits and types? What do they have in common?
// - match branches for variant of an enum. Unlike if-else. Match can recognize errors and a bunch of stuff!
// - Shadowing is like overwriting but scoped?