fn main() {
    let mut words: Vec<String> = Vec::new();

    for arg in std::env::args().skip(1) {
        words.push(arg);
    }
    words.sort();

    for word in words {
        println!("{}", word);
    }
}
