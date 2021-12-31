// Fun with functions

// Function parameters are strongly typed
// Chapter NOTE: In function signatures, you must declare the type of each parameter
fn number_taker(x: i32) {
    println!("number taker recieved parameter x of value {}", x);
}
fn main() {
    println!("Hello, world!: The Start of the program");

    number_taker(5);
}
