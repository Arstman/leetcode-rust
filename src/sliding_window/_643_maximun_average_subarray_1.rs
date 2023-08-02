struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        nums.windows(k as usize)
            .map(|w| w.iter().sum::<i32>())
            .max()
            .unwrap() as f64
            / k as f64
    }
}

/// Top Voted Solution is as above
// struct SolutionTop;

// impl SolutionTop {

// }
// this actually from expample code of Leetcode.cn
struct SolutionTop;

impl SolutionTop {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let (mut l, mut r) = (0, 0);
        let (mut sum, mut avg) = (0., f64::MIN);
        while r - l + 1 != k as usize {
            sum += nums[r] as f64;
            r+=1;
        }
        while r < nums.len() {
            sum += nums[r] as f64;
            avg = avg.max(sum / k as f64);
            r += 1;
            sum -= nums[l] as f64;
            l += 1;
        }
        avg
    }
}


#[test]
fn test_643() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    let res = 12.75;
    assert_eq!(Solution::find_max_average(nums, k), res);

    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 2;
    let res = 26.5;
    assert_eq!(Solution::find_max_average(nums, k), res);

    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 1;
    let res = 50.0;
    assert_eq!(Solution::find_max_average(nums, k), res);
}

#[test]
fn test_643_top() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    let res = 12.75;
    assert_eq!(SolutionTop::find_max_average(nums, k), res);
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 2;
    let res = 26.5;
    assert_eq!(SolutionTop::find_max_average(nums, k), res);
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 1;
    let res = 50.0;
    assert_eq!(SolutionTop::find_max_average(nums, k), res);
}
