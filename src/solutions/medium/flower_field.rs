const FLOWER_CODE: u8 = 0x2A;
const EMPTY_CODE: u8 = 0x20;
const ZERO_CODE: u8 = 0x30;

#[rustfmt::skip]
const NEIGBOURHOOD_OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn count_neigbourhood(garden: &[&str], i: usize, j: usize) -> char {
    let (height, width) = (garden.len(), garden[0].len());
    match NEIGBOURHOOD_OFFSETS
        .iter()
        .map(|&(di, dj)| (i.wrapping_add_signed(di), j.wrapping_add_signed(dj)))
        .filter(|&(i, j)| i < height && j < width && garden[i].as_bytes()[j] == FLOWER_CODE)
        .count()
    {
        0 => EMPTY_CODE as char,
        n => (n as u8 + ZERO_CODE) as char,
    }
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.bytes()
                .enumerate()
                .map(|(j, col)| {
                    if col == FLOWER_CODE {
                        FLOWER_CODE as char
                    } else {
                        count_neigbourhood(garden, i, j)
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_rows() {
        let input = &[];
        let expected: &[&str] = &[];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn no_columns() {
        let input = &[""];
        let expected = &[""];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn no_flowers() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        "   ",
        "   ",
    ], &[
        "   ",
        "   ",
        "   ",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn garden_full_of_flowers() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "***",
        "***",
    ], &[
        "***",
        "***",
        "***",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn flower_surrounded_by_spaces() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        " * ",
        "   ",
    ], &[
        "111",
        "1*1",
        "111",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn space_surrounded_by_flowers() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "* *",
        "***",
    ], &[
        "***",
        "*8*",
        "***",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn horizontal_line() {
        let input = &[" * * "];
        let expected = &["1*2*1"];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn horizontal_line_flowers_at_edges() {
        let input = &["*   *"];
        let expected = &["*1 1*"];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn vertical_line() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        " ",
        "*",
        " ",
        "*",
        " ",
    ], &[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn vertical_line_flowers_at_edges() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "*",
        " ",
        " ",
        " ",
        "*",
    ], &[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn cross() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "  *  ",
        "  *  ",
        "*****",
        "  *  ",
        "  *  ",
    ], &[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn large_garden() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        " *  * ",
        "  *   ",
        "    * ",
        "   * *",
        " *  * ",
        "      ",
    ], &[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
}
