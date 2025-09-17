Rust has `String` and `&str` to represent string.


<h5> `&str` </h5>
`&str`: a string reference, consistsing only a pointer to a starting position in memory and a length. It's can be stack memory or heap memory, anywhere in memory. It can be backed by a stack-allocated array buffer, or string literal compiled into program binary itself (heap).

`&'static str`: a new syntax here (`'static`) is  <em>lifetime annotation</em>, which means it's will live for the entire runtime of program. The string will be complied in the binary, so it can be referred anytime!. Since this variable will be saved in read-only data segment (.rodata) in memory, this variable cannot be modified.

Example 1:
```
struct Artwork {
    name: &'static str,
}

fn admire_art(art: &Artwork) {
    print_admiration(art.name);
}

fn print_admiration(name: &str) {
    println!("Wow, {} really makes you think", name);
}

fn main() {
    let art1 = Artwork{name: "My selfie"}; // no need // to_string()  because it's declared as literal str
    admire_art(&art1)
}

```

- "My selfie" is a literal string, so it will be put at read-only data segment (.rodata) and last there untill the program finish.
- `art1` is a struct, will be allocated in stack memory when the program runs, and `name` field is a pointer and lenght, point to the literal "My selfie"

Since `art1.name` is just a pointer to a data segment, so if the art1 is moved, we can still can create another variable to point to that "My selfile" literal:
```
struct Artwork {
    name: &'static str,
}

fn main() {
    let art1 = Artwork { name: "My selfie" };

    // move ownership to art2
    let art2 = art1;

    // art1 is invalidated, so we can not use it anymore but we still can create a variable to point it to literal
    // In this example, "My selfie" is only one string literal in the compiled binary, even though we write it twice in the source code.
    let art3 = Artwork { name: "My selfie" };

    println!("{}", art2.name);
    println!("{}", art3.name);
}

```
The compiler will store the UTF-8 bytes of "My selfie" once in the program’s read-only data segment (.rodata). Every time we write "My selfie" in code, the compiler just creates a reference (pointer + length) to the same static bytes.