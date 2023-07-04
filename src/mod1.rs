pub fn print_characters() {
    println!("a to Z character: ");
    for c in ('Z'..='a').rev() {
        println!("{}", c);
    }
}
