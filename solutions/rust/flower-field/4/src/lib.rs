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