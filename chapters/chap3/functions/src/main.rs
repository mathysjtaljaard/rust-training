// Fun with functions

// Function parameters are strongly typed
// Chapter NOTE: In function signatures, you must declare the type of each parameter
fn number_taker(x: i32) {
    println!("number taker recieved parameter x of value {}", x);
}

fn advanced_taker(x: i32, y: char) {
    println!("advanced taker recieved x {} and y {} ", x, y);
}

fn something_diff() {
    // Rust doesn't allow this type of assignments
    //let x = y = 6;

    let x = 6;
    let y = 6;

    println!("x {}, y {}", x, y);
}

fn creating_new_scope_and_wait_what() {
    let y = {
        let x = 3;
        //WAIT WHAT, NO ENDING SEMI COLON???
        x + 1 // BY Not adding the semi colon, this is considered an expression
              // if we add a semi colon it is considered an statement
    };

    // this is a statement inner block. Can you call it?
    let x = {
        let z = 8;
        println!("z is {}", z);
    };

    println!("outer scope y = {}", y);
    // More analysis in the future
    println!("what is x considred to be?? = {:#?}", x);
}

fn time_to_return() -> i64 {
    // again no semi colon :whatface:
    5
}

fn assign_that_non_colon() {
    let x = time_to_return();
    println!("x is assigned the value {} by calling time_to_return", x);
}

fn main() {
    println!("Hello, world!: The Start of the program");

    number_taker(5);
    advanced_taker(10, 'C');
    something_diff();
    creating_new_scope_and_wait_what();
    assign_that_non_colon();
}
