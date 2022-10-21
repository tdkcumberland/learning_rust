#[derive(Debug)]
pub struct HighScores<'a>{
    score_list: &'a[u32]
}

impl <'a> HighScores<'a>{
    pub fn new(scores: &'a[u32]) -> Self {
        Self {
            score_list: scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.score_list
    }

    pub fn latest(&self) -> Option<u32> {
        match self.score_list.last() {
            Some(a) => Some(*a),
            None => None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.score_list.iter().max() {
            Some(a) => Some(*a),
            None => None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut vec = self.score_list.to_vec();
        vec.sort_unstable_by(|a,b| b.cmp(a));
        // this is much safer than vec[..3].to_vec() as it can handle smaller size or empty
        // vec[..3] will panic when trying to access non existent indices
        vec.truncate(3); 
        vec
    }
}
