fn main(){
    // * usually rust compiler can guess a type by the way we use it in our program
    // * however, in case when one can have many types, we are expected to do type annotation
    let guess: u32 = "42".parse().expect("Not a number!");  // would have an error without type annotation

    // * for making the bit size according to the architecture, use isize or usize
    
    let sum = 5.0 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;


    // * rust char type
    // * they use single quotation for char's. they are of size 4 bytes and represent a unicode scalar value
    // * it can represent chinese, japanse etc


    // * Compound Types
    // to represent multiple values together

    // * tuple: (*datatypes)

    let tup: (i32, f64, u8) = (500, 4.3, 1);

    // array
    // * unlike python, arrays are fixed-length. 
    // * to decalre type and size for array, use square bracket with type and size

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // * to initialize an array with all same values:__rust_force_expr:
    let arr = [3; 5];

    // * array indexing
    print!("{}", arr[0]);

}