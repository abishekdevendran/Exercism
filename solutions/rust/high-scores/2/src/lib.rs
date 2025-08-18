use std::cmp::Ordering;

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut ans = Vec::with_capacity(3);

        for &score in &self.scores {
            if let Some(pos) = ans.iter().position(|&x| x < score) {
                ans.insert(pos, score);
            } else if ans.len() < 3 {
                ans.push(score);
            }

            if ans.len() > 3 {
                ans.pop();
            }
        }

        ans.clone()
    }
}
