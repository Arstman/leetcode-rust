struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for i in 0..nums.len() {
            if let Some(&v) = map.get(&nums[i]) {
                if i - v as usize <= k as usize {
                    return true;
                }
            }
            map.insert(nums[i], i as i32);
        }
        false
    }
}

/// Top voted solution
use std::collections::HashMap;

struct SolutionTop;

impl SolutionTop {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        // [1] all encountered numbers with their corresponding
        //     indices will be stored in a HashSet
        let mut seen = HashMap::with_capacity(nums.len());

        // [2] in this cycle, we both check for correct
        //     duplicates and update the dictionary
        for (i, n) in nums.into_iter().enumerate() {
            let j = seen.entry(n).or_insert(i);
            if *j != i && i - *j <= k as usize {
                return true;
            } else {
                *j = i;
            }
        }

        return false;
    }
}

#[test]
fn test_219() {
    assert_eq!(
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
        true
    );
    assert_eq!(
        Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
        true
    );
    assert_eq!(
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
        false
    );
    assert_eq!(
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 3),
        true
    );
}

#[test]
fn test_219_top() {
    assert_eq!(
        SolutionTop::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
        true
    );
    assert_eq!(
        SolutionTop::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
        true
    );
    assert_eq!(
        SolutionTop::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
        false
    );
    assert_eq!(
        SolutionTop::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 3),
        true
    );
}
