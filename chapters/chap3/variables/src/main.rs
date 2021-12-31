fn cannot_work() {
    println!("Hello, world!");

    let x = 5;
    println!("value of x == {}", x);

    // x = 6;
    // println!("value of x == {}", x);
}

fn can_work() {
    println!("Hello, world!");

    let mut x = 5;
    println!("value of x == {}", x);

    x = 6;
    println!("value of x == {}", x);
}

fn constant_variable() {
    const VALUE: &str = "Value is Constant";
    println!("{}", VALUE);
}

fn variable_shadowing() {
    let x = 5;

    let x = x + 1;

    {
        //inner scope
        let x = x * 2;

        println!("Value of inner scope x {}", x);
    }

    println!("Value of x {} ", x);
}

fn strange_() {
    let spaces = "  ";
    // this will be a compiler error
    // error: mismatched types
    // label: expected `&str`, found `usize`
    // This will error due to trying to assign a new value to an immutable object
    //spaces = spaces.len();
    println!("length {}", spaces.len());
}

fn assign_str_len() {
    let spaces = "  ";
    println!("{}", spaces);
    // This is allowed due to shadowing,
    let spaces = spaces.len();
    println!("{}", spaces);
}

fn number_ops() {
    let sum = 5 + 11;
    let diff = 95.4 - 4.5;
    // cannot multiply a float with an int?
    // how do you do that? some casting or something?
    let product: f32 = 5.5 * 3.3;
    // cannot divide a integer with a float and vice versa
    // how do you do that
    let quotient = 55.3 / 32.3;
    let floored = 2 / 3;
    let remainder = 40 % 3;

    println!(
        " sum {} , diff {} , product {}, quotient {}, floored {}, remainder {} ",
        sum, diff, product, quotient, floored, remainder
    );
}

fn bolean_time() {
    let t = true;
    let f: bool = false;

    println!("t {}, f {}", t, f);
}

fn character_time() {
    let c = 'x';
    let z = 'ℤ';
    let cat = '😻';

    println!("c {}, z {}, cat {}", c, z, cat);
}

fn someone_drop_a_tuple() {
    let tup = (400, 3.23, 'z');
    println!(
        "tup index 0 {}, tup index 1 {}, tup index 3 {} ",
        tup.0, tup.1, tup.2
    );
    // deferernincing a tuple (similar to javascript ecma script)
    let (x, y, z) = tup;
    println!("x {}, y {}, z {}", x, y, z);
}

fn array_in_your_future() {
    let a = [1, 2, 3, 4, 5];
    // So to print an array, you will need to add additional instructions to the String interpeter
    //      ?? is that what it's called?
    println!("array {:#?}", a);

    // explicitly defining the type and size of the array (cannot add more than defined array size)
    let b: [i32; 6] = [1, 2, 3, 4, 5, 6];
    //shodow b and make it characters
    //let b = ['a','b'];
    //NOTE: when trying to access array indexes by using an user input (or any input)
    // the code will panic if the entered index value > size of the array..
    println!("b {:?}", b);
}
// So the location of the main function doesn't matter?
// no it doesn't
fn main() {
    cannot_work();
    can_work();
    constant_variable();
    variable_shadowing();
    strange_();
    assign_str_len();
    number_ops();
    bolean_time();
    character_time();
    someone_drop_a_tuple();
    array_in_your_future();
}
