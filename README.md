# [Rust Programming Language Site](https://doc.rust-lang.org/book/title-page.html)

# Training Notes: 

## Chapter 1

1. Using the `!` indicates rust is calling a marco and not function
2. function body consists of 
```rust 
    fn name(parameters) {
        //use of semi colon required 
        // can use rustfmt to format (feels similar to golang)
    }
```
3. Generating a new project use `Cargo` 
```cmd
    cargo new projectname
``` 
> > new project will contain folders `src/, .gitignore, Cargo.toml, Cargo.lock, target` 
4. TOML -> `Tom's Obvious, Minimal Language`
5. Compiling a rust project 
   1. Using rust cli
      1. `cmd: rustc main.rc`
      2. `cmd: ./main`
   2. Using cargo cli
      1. `cmd: cargo build` -> generates binary in directory  `./target/debug/yourprjectname`
      2. `cmd: cargo run` 
6. If you want to verify code compiles, but **NOT** generate executable
    1. `cmd: cargo check`
7. Ready for a release `:-)` then run
   1. `cmd: cargo build --release`
8. If you get an error message (vscode) analyzer cannot find workspace
    1. update settings.json to include **(Example)**
    ```json
        "rust-analyzer.linkedProjects": [
			"chapters/chap1/proj1_hello_world/Cargo.toml"
		]
    ```

## Chapter 2 -> [Notes in src code](chapters/chap2/guessing_game/src/main.rs)

## Chapter 3 -> [Notes in src code](chapters/chap3/variables/src/main.rs)