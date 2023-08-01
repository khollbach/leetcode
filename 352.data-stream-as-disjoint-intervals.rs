/*
 * @lc app=leetcode id=352 lang=rust
 *
 * [352] Data Stream as Disjoint Intervals
 */

// @lc code=start

/// Upper limit, exclusive.
const N: usize = 10usize.pow(4) + 1;

struct SummaryRanges {
    seen: Box<[bool; N]>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            seen: Box::new([false; N]),
        }
    }

    fn add_num(&mut self, value: i32) {
        assert!(0 <= value && value < N as i32);
        self.seen[value as usize] = true;
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut out = vec![];

        let mut indices = 0..N;

        while let Some(start) = indices.find(|&i| self.seen[i]) {
            let end = indices.find(|&i| !self.seen[i]).unwrap_or(N);

            let end_inclusive = end - 1;
            out.push(vec![start as i32, end_inclusive as i32]);
        }

        out
    }
}

// @lc code=end
