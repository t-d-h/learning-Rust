<h2>Ownership and lifetime of a variable</h2>
There are some rules about variables in Rust:
- Each value has a owner, it's the variable
- One value only has one owner at a time, it means that the owner of a value can be changed.
- If the owner goes out of scope, the value is dropped.

Example code:
```
struct Artwork { // same as class in other languages, but do not support inheritance.
    name: String,
}

fn admire_artwork(art: Artwork) {
    println!("Wow, it's {}", art.name);
}

fn main() {
    let art1 = Artwork{ name: "Mona lisa".to_string() }; //we use let to init a variable, to_string() is to convert &str to String, will learn latter.
    admire_artwork(art1)
} 
```
Result:
```
$ cargo run
   Compiling ownership-and-borrowing v0.1.0 (/home/h0antran/learning-Rust/001-ownership-and-borrowing)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/ownership-and-borrowing`
Wow, it's Mona lisa
```
It works! But if we want to run `admire_artwork(art1)` twice like:
```
$ cargo run
   Compiling ownership-and-borrowing v0.1.0 (/home/h0antran/learning-Rust/001-ownership-and-borrowing)
error[E0382]: use of moved value: `art1`
  --> src/main.rs:12:20
   |
10 |     let art1 = Artwork{ name: "Mona lisa".to_string() }; //we use let to init a variable, to_string() is to convert &str to String, will learn latter.
   |         ---- move occurs because `art1` has type `Artwork`, which does not implement the `Copy` trait
11 |     admire_artwork(art1);
   |                    ---- value moved here
12 |     admire_artwork(art1)
   |                    ^^^^ value used here after move
   |
note: consider changing this parameter type in function `admire_artwork` to borrow instead if owning the value isn't necessary
  --> src/main.rs:5:24
   |
5  | fn admire_artwork(art: Artwork) {
   |    --------------      ^^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
note: if `Artwork` implemented `Clone`, you could clone the value
  --> src/main.rs:1:1
   |
1  | struct Artwork { // same as class in other languages, but do not support inheritance.
   | ^^^^^^^^^^^^^^ consider implementing `Clone` for this type
...
11 |     admire_artwork(art1);
   |                    ---- you could clone this value

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership-and-borrowing` (bin "ownership-and-borrowing") due to 1 previous error
```

Explanation:
When we ran `admire_artwork(art1)` the first time, the ownership of art1 value `"Mona Lisa"` was transferred, from `art1` in main function to `art` in `admire_artwork` function. After the first call, `art1` is no longer valid because it no longer refers to anything that can be used because when `admire_artwork` is completed, the memory that stores value of `art1` was `deallocated`.

If we write this example in other language, the program may run success fully because the lifetime of `art1` can last untill the programm is completed. But in Rust, because the ownership was moved to `art` in `admire_artwork`, so it will be unvalidated and Rust will deallocate the memory for its value. 
In short, lifetime of a variable in Rust ends when its "parent" function complete.
