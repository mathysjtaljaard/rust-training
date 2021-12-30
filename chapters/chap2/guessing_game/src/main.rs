// NOTE: by default rust imports std::prelude https://doc.rust-lang.org/std/prelude/index.html#prelude-contents
// since std::prelude DOESN'T contain io we need to import it manually
use std::io;

// NOTE: Rust doesn't include random number functionality in its standard library
//  but you can use https://crates.io/crates/rand
// Crate is a collection of Rust source code files (java jar)
// Crates are added to the Cargo.toml file
// after adding the dependency(ies) you need to run cargo build
use rand::Rng;

// NOTE: Ordering is used as part of the cmp functionality for ordering and comparison
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // NOTE: gen_range(1..=100) is between inclusive 1 and inclusive 100
    //  or you can set it as 1..101 == between inclusive 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut number_of_tries: u32 = 0;
    println!("Please input your guess.");

    loop {
        number_of_tries += 1;
        //Creating variables
        // let apple = 5; //NOTE: this is immutable -> java final, javascript const
        // let values = [1, 2, 3, 4];
        // let obj = {"runner": {"place": 3}};
        let mut guess = String::new(); //NOTE: this is mutable
                                       // NOTE :: in states that new is an associated function of the type String

        io::stdin()
            // NOTE: & is a `reference` to the variable. This allows for a reference pointer to the variable
            //  and not needing to copy the value.. (Pass by reference instead of pass by value?)
            .read_line(&mut guess)
            // read_line returns an value of io::Result https://doc.rust-lang.org/std/io/type.Result.html
            // Result is considered an enumaration https://doc.rust-lang.org/book/ch06-00-enums.html
            .expect("Failed to read line");

        // NOTE: curly brackets are place holders for your values to be added within the println macro
        // println!(
        //     "You guessed: {} The Secret number is {}",
        //     guess, secret_number
        // );

        // variable shadowing is allowed by rust (we have the mutable guess variable defined)
        // let guess: i32 = guess.trim().parse().expect("Please give a number");
        // Error Handler (Result is Enum of Ok, Err)
        // when we get a number return it
        // NOTE: you need to add a match (ontrol flow based on pattern matching.)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ underscore is a catch all (go uses it as an ignore value)
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Equal => {
                println!("Winner Winner, Chicken Dinner!!!");
                break;
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }
    }

    println!("It only took you {} tries to win!!", number_of_tries);
}
