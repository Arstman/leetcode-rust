struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut small = i32::MAX;
        let mut big = i32::MAX;
        for n in nums {
            if n <= small {
                small = n;
            } else if n <= big {
                big = n;
            } else {
                return true;
            }
        }
        return false;
    }
}

struct Solution1;

impl Solution1 {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        nums.iter()
            .scan((i32::MAX, i32::MAX), |(small, big), &n| {
                if n <= *small {
                    *small = n;
                } else if n <= *big {
                    *big = n;
                } else {
                    return Some(true);
                }
                return Some(false);
            })
            .any(|x| x)
    }
}

/// Top Voted Solution
struct SolutionTop;

impl SolutionTop {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        // [1] there exist a triple of increasing numbers if some
        //     number 'n' is greater than another number 'second'
        //     that itself is greater than yet another number 'first'

        let mut first = i32::MAX;
        let mut second = i32::MAX;

        // [2] in the cycle, we iteratively update 'first' and 'second'
        //     numbers while waiting for the third one to appear
        for n in nums {
            if n <= first {
                first = n;
            } else if n <= second {
                second = n;
            } else {
                return true;
            }
        }

        return false;
    }
}

#[test]
fn test_334() {
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::increasing_triplet(nums), true);

    let nums = vec![5, 4, 3, 2, 1];
    assert_eq!(Solution::increasing_triplet(nums), false);

    let nums = vec![2, 1, 5, 0, 4, 6];
    assert_eq!(Solution::increasing_triplet(nums), true);

    let nums = vec![5, 1, 5, 5, 2, 5, 4];
    assert_eq!(Solution::increasing_triplet(nums), true);
}

#[test]
fn test_334_1() {
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution1::increasing_triplet(nums), true);
    let nums = vec![5, 4, 3, 2, 1];
    assert_eq!(Solution1::increasing_triplet(nums), false);
    let nums = vec![2, 1, 5, 0, 4, 6];
    assert_eq!(Solution1::increasing_triplet(nums), true);
    let nums = vec![5, 1, 5, 5, 2, 5, 4];
    assert_eq!(Solution1::increasing_triplet(nums), true);
}

#[test]
fn test_334_top() {
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(SolutionTop::increasing_triplet(nums), true);
    let nums = vec![5, 4, 3, 2, 1];
    assert_eq!(SolutionTop::increasing_triplet(nums), false);
    let nums = vec![2, 1, 5, 0, 4, 6];
    assert_eq!(SolutionTop::increasing_triplet(nums), true);
    let nums = vec![5, 1, 5, 5, 2, 5, 4];
    assert_eq!(SolutionTop::increasing_triplet(nums), true);
}
