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


<h5> `String` </h5>
or owned string is made up of a growable, heap-allocated buffer that stores the character data. Unlike language like Python, when you concatenate a string onto a string, it have to reallocate to other larger storage area, which make some cost for allocation.
In Rust, if we add a new character to the end of the string, we can add it to the end of the buffer, or moving around some character inside the string without reallocation as long as we dont exceed the capacity of the current buffer.

Some ways to declare a string:
```
let mut x = String::new(); //create a mew string with a buffer that has capacity of zero, this function does not perform any allocations.

let mut y = String::with_capacity(10_000_000); // to explicitly set capacity up front, so reduce allocations later. If we working with large strings, this leads to a large performance gain
```

<h5> String and &str </h5>
- String: own the data, header in stack memory and real data in heap 
- &str:  just a pointer, data may lies in stack/heap/static

Converting data from `String` to `&str` is very cheap. Since `&str` values are simply a pointer and length, so we can just copy the starting pointer of the `String`'s buffer and its length:
```
fn main {
    let value = String::new();
    print_admiration(value.as_str())
}

fn print_admiration(name: &str) {
    println!("{} is great", name);
}

// String value .as_str() -> &str 
```

On the contrary, converting from `&str` to `String` is more expensive, since `String` has its own the heap-allocated buffer, so the computer has to allocate a buffer at least large enough to hold all the data that `&str` referred to, then clone these data to new allocated buffer area.

```
fn main {
    let value2= "abc";
    print_admiration2(value2.to_string())
}

fn print_admiration2(name: String) {
    println!("{} is great", name);
}

//  &str .to_string() -> String 
```
TLDR:
`as_` generally means we are getting a cheap reference to some things.
`to_` means we are allocating and copying to an ownded data structure.

knowing to use which string type comes with exp, to generalize:
- data in a struct, which will live for a long time: use `String`
- just passing read-only data to a function, probaly `&str`
- If not sure, use `String`
