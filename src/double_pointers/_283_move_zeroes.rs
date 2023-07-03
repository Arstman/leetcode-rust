struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() == 1 {
            return;
        }
        let mut non_zero_index = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[non_zero_index] = nums[i];
                non_zero_index += 1;
            }
        }
        if non_zero_index == 0 {
            return;
        }
        for i in non_zero_index..nums.len() {
            nums[i] = 0;
        }
    }
}

/// Top Voted Solution
struct SolutionTop;

impl SolutionTop {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut z = Vec::new();
        nums.retain(|x| {
         if *x != 0 { true } else { z.push(0); false }
        });
       let _ = &nums.append(&mut z);
    }
}

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

#[test]
fn test_283_top() { 
    let mut nums = vec![0, 1, 0, 3, 12];
    SolutionTop::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    let mut nums = vec![0];
    SolutionTop::move_zeroes(&mut nums);
    assert_eq!(nums, vec![0]);
    let mut nums = vec![1];
    SolutionTop::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1]);
    let mut nums = vec![0, 0, 0, 0, 0];
    SolutionTop::move_zeroes(&mut nums);
    assert_eq!(nums, vec![0, 0, 0, 0, 0]);
}
