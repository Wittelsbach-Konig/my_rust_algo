#[derive(Debug)]
pub struct HighScores<'a> {
    _scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { _scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self._scores
    }

    pub fn latest(&self) -> Option<u32> {
        self._scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self._scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut res_vec = self._scores.to_vec();
        res_vec.sort_by(|a, b| b.cmp(a));
        res_vec.truncate(3);
        res_vec
    }
}