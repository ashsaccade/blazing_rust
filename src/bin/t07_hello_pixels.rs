fn main() {}

pub fn parse_bitmap_8x8(lines: [&str; 8]) -> [u8; 8] {
    let mut result = [0u8; 8];

    for (i, line) in lines.iter().enumerate() {
        let bytes = line.as_bytes();

        let mut byte: u8 = 0b0000_0000;

        for b in bytes {
            byte = byte >> 1;
            if *b == b'#' {
                byte |= 0b1000_0000;
            }
        }
        result[i] = byte
    }

    result
}

pub fn render_bitmap_8x8(bytes: [u8; 8]) -> [String; 8] {
    let mut result: Vec<String> = Vec::with_capacity(8);
    // let mut result: [String; 8] = Default::default();

    for byte in bytes {
        let mut line = String::new();

        for j in 0..8 {
            let bit = (byte >> (7 - j)) & 1;

            if bit == 1 {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        result.push(line);
    }

    result.try_into().unwrap()
}

pub fn invert_bitmap_8x8(bytes: [u8; 8]) -> [u8; 8] {
    // in progress
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bitmap() {
        #[rustfmt::skip]
        let image = [
          "..####..",
          ".#....#.",
          "#.#..#.#",
          "#..##..#",
          "#......#",
          "#.#..#.#",
          ".#....#.",
          "..####..",
        ];
        let res = parse_bitmap_8x8(image);
        assert_eq!(
            0b0011_1100, res[0],
            "expected {:08b} but got {:08b}",
            0b0011_1100, res[0]
        )
    }

    #[test]
    fn render_bitmap() {
        #[rustfmt::skip]
        let image = [
          "..####..",
          ".#....#.",
          "#.#..#.#",
          "#..##..#",
          "#......#",
          "#.#..#.#",
          ".#....#.",
          "..####..",
        ];
        let bytes = [
            0b0011_1100,
            0b0100_0010,
            0b1010_0101,
            0b1001_1001,
            0b1000_0001,
            0b1010_0101,
            0b0100_0010,
            0b0011_1100,
        ];
        let res = render_bitmap_8x8(bytes);
        assert_eq!(image, res)
    }
}
