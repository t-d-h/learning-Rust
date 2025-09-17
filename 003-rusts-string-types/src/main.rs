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