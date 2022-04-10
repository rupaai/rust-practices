fn main() {
    // * every variable has to be declared with let. If datatype is not declared, rust infers the type automatically
    // * all variables are immutable unless explicitly mentioned.
    let x = 5; 
    println!("The value of x is: {}", x);
    // x = 6;
    // ! the above code will throw error as x is immutable


    // * immutable variable
    // * when we use large data structures, mutability is better than continously copying the same variables and allocating.
    let mut y = 4;
    y = 3;

    // * shadowing
    // * it is a constant to change a mutable variable
    // * usually, when a variable has to be changed a few times, it is better to apply this technique instead of mutability
    // * in case of shadowing, we can even change the type of the variable. which is not possible for mut.
    let x = 45;
    let x = x + 1;

    // * constnats
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // * no need of let while declaring constants

    
    // * to understand why mutability is not recommended, the below link might help
    // https://stackoverflow.com/questions/214714/mutable-vs-immutable-objects
 
}