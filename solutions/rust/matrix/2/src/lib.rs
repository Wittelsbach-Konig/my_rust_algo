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
        self.matrix.get(row_no.checked_sub(1)?).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let col = col_no.checked_sub(1)?;
        (col < self.matrix[0].len()).then(|| self.matrix.iter().map(|row| row[col]).collect())
    }
}