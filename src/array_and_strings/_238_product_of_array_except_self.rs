struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(1, |acc, &x| {
                let tmp = *acc;
                *acc *= x;
                Some(tmp)
            })
            .zip(
                nums.iter()
                    .rev()
                    .scan(1, |acc, &x| {
                        let tmp = *acc;
                        *acc *= x;
                        Some(tmp)
                    })
                    .collect::<Vec<i32>>()
                    .iter()
                    .rev()
            )
            .map(|(a, b)| a * b)
            .collect()
        }
}

/// Top Voted Solution
struct SolutionTop;

impl SolutionTop {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![1; nums.len()];
        let mut l = 0;
        let mut r = nums.len() - 1;

        let mut lv = 1;
        let mut rv = 1;
        loop {
            ret[l] = ret[l] * lv;
            ret[r] = ret[r] * rv;

            rv = rv * nums[r];
            lv = lv * nums[l];

            if r == 0 {
                break;
            }
            l += 1;
            r -= 1;
        }

        return ret;
    }
}


#[test]
fn test_238() {
    let nums = vec![1, 2, 3, 4];
    let res = vec![24, 12, 8, 6];
    assert_eq!(Solution::product_except_self(nums), res);

    let nums = vec![-1, 1, 0, -3, 3];
    let res = vec![0, 0, 9, 0, 0];
    assert_eq!(Solution::product_except_self(nums), res);

    let nums = vec![0, 0];
    let res = vec![0, 0];
    assert_eq!(Solution::product_except_self(nums), res);
}

#[test]
fn test_238_top() {
    let nums = vec![1, 2, 3, 4];
    let res = vec![24, 12, 8, 6];
    assert_eq!(SolutionTop::product_except_self(nums), res);

    let nums = vec![-1, 1, 0, -3, 3];
    let res = vec![0, 0, 9, 0, 0];
    assert_eq!(SolutionTop::product_except_self(nums), res);
    
    let nums = vec![0, 0];
    let res = vec![0, 0];
    assert_eq!(SolutionTop::product_except_self(nums), res);
}
