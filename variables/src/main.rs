const PI: f64 = 3.14;

fn main() {
    println!("{PI}");
    valid_mutable();
    const_check();
    shadowing_in_action();
}

fn shadowing_in_action() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of x in inner scope is {}", x);
    }

    println!("Value of x in outer scope is {}", x);
}

// Compiler will first bind x to value 5. When we shadow x, it will take the original x then plus 1 then bind it to variable x.
// When inside inner scope, we shadow x again with original x multiply by 2, then bind it to x. The shadowing of x will end when scope end
// which mean outside the scope, x is now 6

fn const_check() {
    const MY_CONST: bool = true;
    println!("{MY_CONST}");
}

fn valid_mutable() {
    let mut a = 1;
    println!("{a}");
    a = 2;
    println!("{a}");
}

// fn error_immutable() {
//     let a = 1;
//     println!("{a}");
//     a = 2; // Error here, can't re-assign immutable variable
//     println!("{a}");
// }
