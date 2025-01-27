// Do exercises from chapter 3 Rust book

fn main() {
    println!("{}", temperature_converter(21.5, String::from("Fahrenheit")));
    println!("{:.2}", temperature_converter(270.0, String::from("Celsius")));
    for num in 0..10 {
        print!("{} ", fibonacci(num));
    }
    christmas_carol_lyrics();
}

// Ex 3: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn christmas_carol_lyrics() {
    let miracles = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        " partridge in a pear tree"
    ];

    let ordinal_numbers = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    for day in 0..12 {
        let mut lyrics = format!("On the {} day of Christmas,\nmy true love gave to me\n", ordinal_numbers[day]);
        let mut lyrics_miracles = String::from("");
        for miracle in (12-day-1)..12 {
            if miracle == 11 {
                if day == 0 {
                    lyrics_miracles = format!("{}A{}.\n", lyrics_miracles, miracles[miracle]);
                } else if day == 11 {
                    lyrics_miracles = format!("{}And a{}!\n", lyrics_miracles, miracles[miracle]);
                } else {
                    lyrics_miracles = format!("{}And a{}.\n", lyrics_miracles, miracles[miracle]);
                }
            } else {
                lyrics_miracles = format!("{}{},\n", lyrics_miracles, miracles[miracle]);
            }
        }
        lyrics = format!("{}{}", lyrics, lyrics_miracles);
        println!("{}", lyrics);
    }
}

// Ex 2: Fibonacci
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

// Ex 1: Convert temperatures between Fahrenheit and Celsius
fn temperature_converter(temperature: f64, convert_to: String) -> f64 {
    if convert_to == "Fahrenheit" {
        // °F = °C * 1.8 + 32
        println!("Convert from Celsius to Fahrenheit");
        return temperature * 1.8 + 32.0;
    } else if convert_to == "Celsius" {
        // °C = (°F - 32) / 1.8
        println!("Convert from Fahrenheit to Celsius");
        return (temperature - 32.0) / 1.8;
    }

    println!("If you are here then something's wrong with your input");

    return -1.0;
}

