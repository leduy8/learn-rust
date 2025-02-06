// Write a function that takes a string of words separated by spaces 
// and returns the first word it finds in that string. 
// If the function doesn’t find a space in the string, 
// the whole string must be one word, so the entire string should be returned.


// My solution
fn main() {
    let string = String::from("hello from Earth!");
    
    let word = first_word(&string);
    println!("{}", word);
    
    let word = first_word_rust_book(&string);
    println!("{}", word);
}

fn first_word(s: &String) -> String {
    let mut res = String::from("");
    
    for c in s.chars() {
        if c == ' ' {
            return res;
        }

        res.push_str(&c.to_string());
    }

    return res;
}

// Rust book solution
fn first_word_rust_book(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    return &s[..]
}
