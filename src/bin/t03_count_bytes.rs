use std::io::{self, Read};

fn main() {
    let cnt = io::stdin().bytes().count();

    println!("bytes: {cnt}");
}
