struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        candies.iter().map(|c| c + extra_candies >= max).collect()
    }
}


/// best voted Solution
/// ```rust 

/// ```

#[test]
fn test_1431() {
    let result = Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3);
    assert_eq!(result, vec![true, true, true, false, true]);

    let result = Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1);
    assert_eq!(result, vec![true, false, false, false, false]);

    let result = Solution::kids_with_candies(vec![12, 1, 12], 10);
    assert_eq!(result, vec![true, false, true]);
}
