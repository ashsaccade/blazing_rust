fn main() {
    println!("{}", print_main());
}

fn print_main() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_str() {
        assert_eq!(print_main(), "Hello, world!");
    }
}
