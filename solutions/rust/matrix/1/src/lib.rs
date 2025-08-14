pub struct Matrix {
    matrix: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let matrix = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|number| number.parse::<u32>().expect("Invalid number"))
                    .collect()
            })
            .collect();
        Self { matrix }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.matrix.get(row_no.wrapping_sub(1)).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if self.matrix.is_empty() || col_no.wrapping_sub(1) >= self.matrix[0].len() {
            return None;
        }
        Some(
            self.matrix
                .iter()
                .map(|row| row[col_no.wrapping_sub(1)])
                .collect(),
        )
    }
}