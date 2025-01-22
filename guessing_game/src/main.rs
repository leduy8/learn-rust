use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Your guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


// Key takeaways:
// - `std::io` is Rust standard io library
// - `println!`` is a macro that prints the string

// - `let`` is a variable declaration
// - Every variable is immutable by default, to make a var mutable you must set it `mut`
// - `String` is the string type, it's growable, UTF-8 encoded
// - `::` in `::new` indicates the associated function of String type named "new"
// - `let mut guess = String::new();` created a mutable variable that is currently bound to a new, empty instance of a String

// - `read_line` function takes user input and append it (without overriding) it into a mutable string `guess`. The string argument needs to be mutable so the method can change the string's content
// - We send the argument is the references of the variable so it can update content in it's location without copy the content into another location.
// - References are immutable by default
// - Value of `read_line` is an instance of `Result` type. Its value can be `Ok` or `Err`. If it's an `Err`, it needs to be do error handling with `expect` function

// - {} is a placeholader. Usage can be specify it inside the curly brackets or comma-separated. Example: `println!("{x} and {}", y);`

// - `match` is similar to `if-else`. But for working with these cases, `match` is superior
//   - Handling multiple distinct cases.
//   - Working with enums, patterns, or destructured data.
//   - Ensuring exhaustiveness.
//   - Writing cleaner, more expressive code for complex conditions.
// - When using `match`, we expect 2 values should be of the same type. So we need to make a variable to parse it into the same data type
// - Make a new variable shadowing `guess` and convert it from String to u32

// - When we convert. We use trim() and parse()
//   - trim() is needed because when we get user's input. When we type 123 then enter. What program is getting is 123\n (in Windows it can be 123\r\n). So trim() is here to eliminate that
//   - parse() will parse the data type into declared type. In here, we have `:u32` meaning we want to convert into unsigned 32-bit integer.
//   - Additionally, when we create a variable and specify the type after the `:`, Rust will expect the variable to be exactly that type. If we don't, Rust can still detect based on the data we give it.

// We make an infinite loops to only break when we win

// The term `Shadowing` shall be discussed later
