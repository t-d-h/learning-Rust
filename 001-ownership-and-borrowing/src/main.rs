struct Artwork { // same as class in other languages, but do not support inheritance.
    name: String,
}

fn admire_artwork(art: Artwork) {
    println!("Wow, it's {}", art.name);
}

fn main() {
    let art1 = Artwork{ name: "Mona lisa".to_string() }; //we use let to init a variable, to_string() is to convert &str to String, will learn latter.
    admire_artwork(art1);
    admire_artwork(art1)
} 
