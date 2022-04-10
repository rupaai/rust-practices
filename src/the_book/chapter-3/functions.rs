// * the naming convention for functions are snake_case
fn main() {

    // function bodies are madeup of statements and expressions
    // * so far we have seen statements. These don't return any value
    another_function(5);
    let y = {   // * declaring values using a new block
        let x = 3;
        x + 1  // * expressions don't have any ending semicolons
    };

    println!("The value of y is: {}", y);
}

// * in function arguments, you always need to declare the data type
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {   // * in case of returning values, we must declare the returning type
    5  // * when returning, no semicolon is used like expressions
}