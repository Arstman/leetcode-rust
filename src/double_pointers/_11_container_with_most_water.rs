struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = height.len() - 1;
        let mut max_area = 0;
        while start < end {
            let area = (end - start) as i32 * height[start].min(height[end]);
            if area > max_area {
                max_area = area;
            }
            if height[start] < height[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }
        return max_area;
    }
}

/// Top Voted Solution
struct SolutionTop;

impl SolutionTop {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut iter = height.iter().enumerate();
        let mut p1 = iter.next();
        let mut p2 = iter.next_back();
        while let (Some((i, h1)), Some((j, h2))) = (p1, p2) {
            result = result.max(h1.min(h2) * (j - i) as i32);
            if h1 < h2 {
                p1 = iter.next();
            } else {
                p2 = iter.next_back();
            }
        }
        result
    }
}

#[test]
fn test_11() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
    assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
}

#[test]
fn test_11_top() {
    assert_eq!(SolutionTop::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(SolutionTop::max_area(vec![1, 1]), 1);
    assert_eq!(SolutionTop::max_area(vec![4, 3, 2, 1, 4]), 16);
    assert_eq!(SolutionTop::max_area(vec![1, 2, 1]), 2);
}
