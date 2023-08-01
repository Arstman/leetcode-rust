struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut nums = nums;
        // sort first, that is the key
        nums.sort();
        // left and right pointer
        let mut left = 0;
        let mut right = nums.len() - 1;

        // loop while left < right
        while left < right {
            let sum = nums[left] + nums[right];
            // if sum == k, then move both pointers
            if sum == k {
                count += 1;
                left += 1;
                right -= 1;
            } else if sum < k {  // if sum < k, then move left pointer
                left += 1;
            } else {  // if sum > k, then move right pointer
                right -= 1;
            }
        }
        return count;
    }
}

/// Top Voted Solution
struct SolutionTop;
impl SolutionTop {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut map = std::collections::HashMap::with_capacity(nums.len());
        nums.iter().for_each(|&x| {
            let e = map.entry(x).or_insert(0);
            match *e {
                0 => {
                    map.entry(k - x).and_modify(|x| *x += 1).or_insert(1);
                }
                _ => {
                    *e -= 1;
                    count += 1;
                }
            }
        });
        count
    }
}


#[test]
fn test_1679() {
    let nums = vec![1, 2, 3, 4];
    let k = 5;
    let res = 2;
    assert_eq!(Solution::max_operations(nums, k), res);

    let nums = vec![3, 1, 3, 4, 3];
    let k = 6;
    let res = 1;
    assert_eq!(Solution::max_operations(nums, k), res);

    let nums = vec![2, 2, 2, 3, 1, 1, 4, 1];
    let k = 4;
    let res = 2;
    assert_eq!(Solution::max_operations(nums, k), res);

    let nums = vec![1, 1, 1, 1];
    let k = 2;
    let res = 2;
    assert_eq!(Solution::max_operations(nums, k), res);
}

#[test]
fn test_1679_top() {
    let nums = vec![1, 2, 3, 4];
    let k = 5;
    let res = 2;
    assert_eq!(SolutionTop::max_operations(nums, k), res);
    let nums = vec![3, 1, 3, 4, 3];
    let k = 6;
    let res = 1;
    assert_eq!(SolutionTop::max_operations(nums, k), res);
    let nums = vec![2, 2, 2, 3, 1, 1, 4, 1];
    let k = 4;
    let res = 2;
    assert_eq!(SolutionTop::max_operations(nums, k), res);
    let nums = vec![1, 1, 1, 1];
    let k = 2;
    let res = 2;
    assert_eq!(SolutionTop::max_operations(nums, k), res);
}
