fn main(){
    // * usually rust compiler can guess a type by the way we use it in our program
    // * however, in case when one can have many types, we are expected to do type annotation
    let _guess: u32 = "42".parse().expect("Not a number!");  // would have an error without type annotation

    // * for making the bit size according to the architecture, use isize or usize
    
    // ! not possible because of differet types: 
    // ! let sum = 5.0 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;


    // * rust char type
    // * they use single quotation for char's. they are of size 4 bytes and represent a unicode scalar value
    // * it can represent chinese, japanse etc


    // * Compound Types
    // to represent multiple values together

    // * tuple: (*datatypes)

    let tup: (i32, (f64, u8), u8) = (500, (4.3, 2), 1);
    println!("{:?}", tup.1);  
    // ! explanation of above code
    //println! is a macro to do formatted output. {} is used to print a value that implements the Display trait. The error is saying that Timespec does not implement the Display trait, so it cannot be used with {}.

    // You can use {:?} instead of {}. {:?} is used to print a value that implements Debug trait and Timespec implements it.
    // array
    // * unlike python, arrays are fixed-length. 
    // * to decalre type and size for array, use square bracket with type and size

    let _arr: [i32; 5] = [1, 2, 3, 4, 5];

    // * to initialize an array with all same values:__rust_force_expr:
    let arr = [3; 5];
    println!("{}", arr[0]);

    // * array indexing
    println!("{}", arr[0]);

}