#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut foo = Vec::new();
        let mut bar = self.scores.to_owned();
        bar.sort();

        if bar.is_empty() {
            return Vec::new();
        }

        let mut counter = 3;

        if bar.len() < 3 {
            counter = bar.len();
        }

        for _ in 0..counter {
            // foo.push(bar.last().copied().unwrap());
            foo.push(bar.iter().max().copied().unwrap());
            bar.pop();
        }

        foo
    }
}
