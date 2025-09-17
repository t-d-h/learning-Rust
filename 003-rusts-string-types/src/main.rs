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




    let mut x = String::new();
    let mut y = String::with_capacity(10_000_000);
    for i in 0..100000 {
        y.push('.')
    }
    println!("{}", x.len());
    println!("{}", y.len());



    let value = String::new();
    print_admiration(value.as_str());


    let value2= "abc";
    print_admiration2(value2.to_string())
}

fn print_admiration(name: &str) {
    println!("{} is great", name);
}

fn print_admiration2(name: String) {
    println!("{} is great", name);
}
