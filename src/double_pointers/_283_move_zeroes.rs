struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        unimplemented!()
    }
}

/// Top Voted Solution
// struct SolutionTop;

// impl SolutionTop {}

#[test]
fn test_283() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);

    let mut nums = vec![0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![0]);

    let mut nums = vec![1];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1]);

    let mut nums = vec![0, 0, 0, 0, 0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![0, 0, 0, 0, 0]);
}

// #[test]
// fn test_283_top() {}
