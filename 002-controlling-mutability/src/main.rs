struct Artwork {
    view_count: i32,
    name: String
}

fn main() {
    let mut art1= Artwork{name: "Lacasa".to_string(), view_count: 0};
    admire_art(&mut art1); 
    admire_art(&mut art1)
}

fn admire_art(art: &mut Artwork) {   // &mut: mutable reference, which will modify the value. 
    println!("{} people have seen {} to day!", art.view_count, art.name);
    art.view_count +=1;
// reference then mutate then end the reference so at a time, there's only one reference
    println!("{}", build_art().name)
}


fn build_art() -> Artwork {
    let art = Artwork{name: "baba".to_string(), view_count: 0};
    art // <- return keyword can be ommited. the function will return if there's no semicolon at the end of the line.
}
// it's posible to write Rust function to return references, but those functions will usually alsoe take references as input