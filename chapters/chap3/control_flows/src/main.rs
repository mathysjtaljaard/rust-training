fn basic_if(x: i32) {
    if x > 10 {
        println!("this is greater than 10");
    } else {
        println!("this is less than or equal to 10");
    }
}

fn more_advanced_if(x: i16) {
    if x == 0 {
        println!("equal to 0");
    } else if x > 0 {
        println!("greater than 0");
    } else {
        println!("less than 0");
    }
}

fn assign_if_statement_result() {
    let statement = 10;
    let result = if statement > 10 {
        "big man"
    } else {
        "little man"
    };
    println!("result = {}", result);
}

fn loop_ex(x: i8) {
    let mut count = 0;
    loop {
        if count == x {
            break;
        };
        println!("count {} is less than {}", count, x);
        count += 1;
    }
}
/*
   You can optionally specify a loop label on a loop and then use the label with break
   or continue to have those keywords applied to the labeled loop instead of the innermost loop
*/
fn loop_within_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count {} ", count);
        let mut remaining = 10;

        'inner: loop {
            println!("remaining = {} ", remaining);
            if remaining == 8 {
                break 'inner;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn dreaded_out_of_bounds_loop() {
    let mut index = 0;
    let array = [1, 2, 3, 4];

    while index < 4 {
        println!("value of array at index {} is {}", index, array[index]);
        index += 1;
    }
    println!("We made it");
}

fn dreaded_out_of_bounds_loop_little_smarter() {
    let mut index = 0;
    let array = [1, 2, 3, 4];

    while index < array.len() {
        println!("value of array at index {} is {}", index, array[index]);
        index += 1;
    }
    println!("We made it");
}

fn while_you_wait() {
    let mut number = 3;

    while number != 0 {
        println!("waiting for number to hit 0. number is {}", number);
        number -= 1;
    }
    println!(" and we are off to the races.. later");
}

/**
 * As a more concise alternative, you can use a for loop and execute some code for each item in a collection
 */
fn for_loop_values_in_array() {
    let a = [1, 23, 4, 5, 6, 8];

    for value in a {
        println!("the value in array a is {}", value);
    }
}

fn reverse_the_range() {
    for number in (1..=10).rev() {
        println!("Numbers 1-10 reversed {}", number);
    }
}
fn main() {
    println!("Hello, world!");
    basic_if(5);
    more_advanced_if(5);
    more_advanced_if(0);
    more_advanced_if(-1);
    assign_if_statement_result();
    loop_ex(5);
    loop_within_loop();
    while_you_wait();
    dreaded_out_of_bounds_loop();
    dreaded_out_of_bounds_loop_little_smarter();
    for_loop_values_in_array();
    reverse_the_range();
}
