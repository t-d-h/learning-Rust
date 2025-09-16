struct Artwork {
    name: String,
}

fn admire_artwork(art: &Artwork) { 
    println!("Wow, it's {}", art.name);
}


fn remove_artwork(art: Artwork) {
    println!("{} will be removed when this fn complete", art.name);
}


fn main() {
    let art1 = Artwork{ name: "Mona lisa".to_string() }; 
    remove_artwork(art1);
    admire_artwork(&art1)
} 
