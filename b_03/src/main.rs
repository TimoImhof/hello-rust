fn main() {
    // Mutable and Immutable Variables
    let mut x = 5;
    println!("The value of x is: {x}");
    // By default, once we bind a value to a variable, we cannot change it.
    // By adding mut, we make a variable mutable.
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    // Constats are values bound to a name and are not allowed to change.
    // Differnce between constants and variables:
    // 1. Constants are always immutable.
    // 2. The value type must be annotated.
    // 3. Constants can be declared in any scope, including the global scope.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // Naming convention: always uppercase with underscores between words.
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // TODO Next: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
}
