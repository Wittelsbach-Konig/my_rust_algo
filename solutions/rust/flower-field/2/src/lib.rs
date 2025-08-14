const FLOWER_CODE: u8 = 0x2A;
const EMPTY_CODE: u8 = 0x20;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    if height == 0 {
        return Vec::new();
    }

    let width = garden[0].len();
    let bytes: Vec<&[u8]> = garden.iter().map(|s| s.as_bytes()).collect();
    let mut result = Vec::with_capacity(height);

    for y in 0..height {
        let mut row = Vec::with_capacity(width);

        for x in 0..width {
            let cell = bytes[y][x];

            if cell == FLOWER_CODE {
                row.push(FLOWER_CODE);
            } else if cell == EMPTY_CODE {
                let mut count = 0;

                for dy in [-1i32, 0, 1] {
                    for dx in [-1i32, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;

                        if ny >= 0 && ny < height as i32 && nx >= 0 && nx < width as i32 {
                            if bytes[ny as usize][nx as usize] == FLOWER_CODE {
                                count += 1;
                            }
                        }
                    }
                }

                if count == 0 {
                    row.push(EMPTY_CODE);
                } else {
                    row.push(b'0' + count);
                }
            } else {
                row.push(cell);
            }
        }
        result.push(String::from_utf8(row).unwrap());
    }
    result
}
