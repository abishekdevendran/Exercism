use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>,
    high_scores: BinaryHeap<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        println!("DEBUG: {:?}", BinaryHeap::from(scores.to_vec()));
        HighScores { scores: scores.to_vec(), high_scores: BinaryHeap::from(scores.to_vec()) }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.high_scores.peek().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        println!("DEBUG: {:?}", self.high_scores);
        let mut temp = self.high_scores.clone();
        let mut ans = vec![];
        for _ in 0..3{
            if let Some(x) = temp.pop(){
                ans.push(x);
            } else{
                break;
            }
        }
        ans
    }
}
