// NOTE: by default rust imports std::prelude https://doc.rust-lang.org/std/prelude/index.html#prelude-contents
// since std::prelude DOESN'T contain io we need to import it manually
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    //Creating variables
    let apple = 5; //NOTE: this is immutable -> java final, javascript const
                   //let values = [1, 2, 3, 4];
                   //let obj = {"runner": {"place": 3}};
    let mut guess = String::new(); //NOTE: this is mutable
                                   // NOTE :: in states that new is an associated function of the type String

    io::stdin()
        // NOTE: & is a `reference` to the variable. This allows for a reference pointer to the variable
        //  and not needing to copy the value.. (Pass by reference instead of pass by value?)
        .read_line(&mut guess)
        // read_line returns an value of io::Result https://doc.rust-lang.org/std/io/type.Result.html
        // Result is considered an enumaration https://doc.rust-lang.org/book/ch06-00-enums.html
        .expect("Failed to read line");

    // NOTE: 
    println!(
        "You guessed: {}, and other variables created {} need to figure out how to display [1,2,3]",
        guess, apple
    );
}
