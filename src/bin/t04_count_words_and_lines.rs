use std::io::{self, Bytes, Read};

fn main() {
    let bytes = io::stdin().bytes();

    let res = count(bytes);

    println!("result: {res:?}");
}

#[derive(Default, Debug, PartialEq, Eq)]
struct CntResult {
    strs_count: u32,
    words_count: u32,
    bytes_count: u32,
}

fn count(bytes: Bytes<impl Read>) -> CntResult {
    let mut res = CntResult::default();

    let mut in_word = false;

    for b in bytes {
        let byte = b.unwrap();

        println!("{}", byte as char);

        res.bytes_count += 1;

        match byte {
            b'\n' => {
                res.strs_count += 1;
                in_word = false;
            }

            b' ' | b'\t' | b'\r' => {
                in_word = false;
            }

            _ => {
                if !in_word {
                    res.words_count += 1;
                    in_word = true;
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn print_str() {
        assert_eq!(
            CntResult {
                strs_count: 1,
                words_count: 1,
                bytes_count: 4,
            },
            count(Cursor::new("foo\n".as_bytes()).bytes()),
        );

        assert_eq!(
            CntResult {
                strs_count: 1,
                words_count: 1,
                bytes_count: 7,
            },
            count(Cursor::new("  hi  \n".as_bytes()).bytes()),
        );

        assert_eq!(
            CntResult {
                strs_count: 0,
                words_count: 0,
                bytes_count: 0,
            },
            count(Cursor::new("".as_bytes()).bytes()),
        );

        assert_eq!(
            CntResult {
                strs_count: 1,
                words_count: 1,
                bytes_count: 6,
            },
            count(Cursor::new("hello\n".as_bytes()).bytes()),
        );

        assert_eq!(
            CntResult {
                strs_count: 0,
                words_count: 1,
                bytes_count: 5,
            },
            count(Cursor::new("hello".as_bytes()).bytes()),
        );

        assert_eq!(
            CntResult {
                strs_count: 1,
                words_count: 2,
                bytes_count: 11,
            },
            count(Cursor::new("hello rust\n".as_bytes()).bytes()),
        );

        assert_eq!(
            CntResult {
                strs_count: 1,
                words_count: 2,
                bytes_count: 13,
            },
            count(Cursor::new(" hello rust \n".as_bytes()).bytes()),
        );

        assert_eq!(
            CntResult {
                strs_count: 1,
                words_count: 3,
                bytes_count: 5,
            },
            count(Cursor::new("a\tb\nc".as_bytes()).bytes()),
        );
    }
}
