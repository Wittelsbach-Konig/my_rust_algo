#[derive(Debug)]
pub struct HighScores {
    _scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            _scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self._scores
    }

    pub fn latest(&self) -> Option<u32> {
        Some(*self._scores.last()?)
    }

    pub fn personal_best(&self) -> Option<u32> {
        Some(*self._scores.iter().max()?)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self._scores.clone();
        scores.sort_by(|a, b| b.cmp(a));
        if scores.is_empty() {
            vec![]
        } else {
            let third_element = match scores.len() {
                3.. => 3,
                _ => scores.len(),
            };
            scores[..third_element].to_vec()
        }
    }
}
