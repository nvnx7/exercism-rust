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
        &self.scores[..]
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = vec![0; 3];
        let mut i_min: usize;
        let mut v_min: u32;
        for score in &self.scores {
            i_min = 0;
            v_min = top_three[0];
            for i in 0..top_three.len() {
                if v_min > top_three[i] {
                    i_min = i;
                    v_min = top_three[i];
                }
            }

            if score > &top_three[i_min] {
                top_three[i_min] = *score;
            }
        }
        top_three.sort_unstable();
        top_three.into_iter().rev().filter(|&x| x > 0).collect()
    }
}
