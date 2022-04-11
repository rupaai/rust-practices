fn main() {
    if_else();
    looping();
}
fn if_else() {
    let number = 3;  // * In rust, unlike python, there isn't any truthy value for non-booleans
    
    // ! the following line will throw error
    // if number {}
    if 4 < number && number < 8 {  // ! we can't use two comparisons together. we are expected to use && in that case
        print!("Number is greater than or equal to 4, less than 8");
    } else if number < 4 {
        print!("Number is 3");
    }
    else {
        print!("Number is greater than 8");
    }

    // * if condition while initializing with let
    let condition = true;
    let value = 4;
    let some_other_value = 5;
    let number = if condition {value} else {some_other_value};
    println!("The value of number is: {}", number);

    // ! let number = if condition { 5 } else { "six" };
    // * the above code won't work because of having different type
}

fn looping() {
    // loop
    // * one of the main purpose of loop is to use for the operations which require retrying and we know might fail
    // * such as checking whether a thread has finished running a programming
    // *  it will execute the code over and over again
    // * there is no condition
    // * kinda like `while True` of python
    let mut i: i32 = 0;
    loop {
        i = i + 1;
        if i == 3 {
            continue;
        }
        println!("again {}", i);
        if i > 5 {
            break;
        }
    }

    let mut count = 0;
    // * loop labeling
    // * we can break a loop using the label
    'counting_up: loop {  // ! we need to use the apostrophe, otherwise we will get malformed loop label error
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // * we can consider loops as expressions and can return values from loop
    // * we have to return the result after break
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while
    // * unlike python, we will always have a terminating condition here.

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // * while for collections
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // * however this is slow because in the runtime the program needs to check whether index out of bound happens or not
    
    // for
    // let a = [10, 20, 30, 40, 50];
    // * this eliminates the risk of index out of bound error
    // * thus increasing code safety

    for element in a {
        println!("the value is: {}", element);
    }
    for number in 1..4 {  // * range
        println!("{}!", number);
    }
    println!("shoja!!!");

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("reverse!!!");

    
}