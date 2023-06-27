struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        // we need to find `000` in `flowerbed, or `00` in the start or end of `flowerbed
        // for the start `00`, we just assume there is a `0` before the start of `flowerbed`,
        // therefore the count starts from 1; and after search 000 in `flowerbed`, we need to see
        // if remaining count is 2, if so, we can place a flower in the end of `flowerbed`

        let mut n = n;
        let mut count = 1;
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0 {
                count += 1;
            } else {
                count = 0;
            }
            if count == 3 {
                n -= 1;
                count = 1;
            }
        }
        if count == 2 {
            n -= 1;
        }
        return n <= 0;
    }
}

/// 跳格法:
/// 题目要求是否能在不打破规则的情况下插入n朵花，与直接计算不同，采用“跳格子”的解法只需遍历不到一遍数组，处理以下两种不同的情况即可：
/// 【1】当遍历到index遇到1时，说明这个位置有花，那必然从index+2的位置才有可能种花，因此当碰到1时直接跳过下一格。
/// 【2】当遍历到index遇到0时，由于每次碰到1都是跳两格，因此前一格必定是0，此时只需要判断下一格是不是1即可得出index这一格能不能种花，如果能种则令n减一，然后这个位置就按照遇到1时处理，即跳两格；如果index的后一格是1，说明这个位置不能种花且之后两格也不可能种花（参照【1】），直接跳过3格。
/// 当n减为0时，说明可以种入n朵花，则可以直接退出遍历返回true；如果遍历结束n没有减到0，说明最多种入的花的数量小于n，则返回false。   
///
struct Solution1;
impl Solution1 {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut index = 0;
        while index < flowerbed.len() {
            if flowerbed[index] == 1 {
                index += 2;
            } else {
                if index == flowerbed.len() - 1 || flowerbed[index + 1] == 0 {
                    n -= 1;
                    index += 2;
                } else {
                    index += 3;
                }
            }
        }
        return n <= 0;
    }
}

/// Top Voted Solution
/*
 This one started off ugly for me. Maybe the test cases provide some insight there. I knew I was lacking on some edge cases and was able to easily create the representative test cases, which saved me from errant submissions.

 I began naively with a scan method tracking for 3 consecutive zeros. I think I had the intuition that a fold would work but ended up fighting with type mismatches and didn’t see the obvious - initial value, accumulator, and return value of the closure must match. Scan is iter -> iter so return type can deviate from the internal state, so I fell back to that plan. After writing it out, I ended up with a scan into a fold.

 I was still missing the edge cases though, for arrays starting and ending with two 0’s, as well as not properly “making room” for sharing a padding zero between sequences of 3 consecutive (i.e. 0 - 0 - 0 - 0 - 0 in the middle of an array can hold at least 2 flowers, “sharing” the middle 0). I modified the closure logic to reset consecutive to 1 instead of 0 which helped with the middle but not the ends.

 Eventually setting the starting to condition 1 got me halfway there, and I had the idea to pad the whole array with 0s on the ends which would give a similar result, rather than awkwardly checking for beginning and end of the iterator (I very much did not want to write this way). The iter::once chain isn’t much better I’ll admit, but it at least rescues the whole iterator approach.

 I finally got it working after fighting with types removing the iter::once chain from the beginning (the 1 starting condition for consecutive_zeros performs the same function - maybe there is a smoother way to do a similar trick at the end with the consecutive_zeros return value from fold), then ended up completely refactoring the scan closure in favor of fold which allowed me to remove all the mutations (Just return a tuple in the fold, dummy! :D )

 Finally I removed one clumsy escape gate (if n + sum of the array is > half of its length, return false) and moved the other into the overall return expression: if n == 0 { return true; } -> n == 0 || which works the same as a short-circuit

 I feel like this one caused me more difficulty than it should have in working out the mechanics of the iterators in the solution, and as such I’m pretty pleased with how it turned out.

 Do you like the iter::once chain? How would you do it differently with iterators? (also once again feeling the lack of reduce in this ancient Rust version :( )
*/

/*
[1,0,0,0,1]
1
[1,0,0,0,1]
2
[1,0,0,0,0,1]
1
[1,0,0,0,0,1]
2
[1,0,0,0,0,0,1]
2
[1,0,0,0,0,0,1]
3
[1,0,1,0,0,1,0,1,0,0,1]
1
[1,0,1,0,0,1,0,1,0,0,1]
0
[0,0,0,0,0,0,0,0,0,0,0,0,0,0]
7
[0,0,0,0,0,0,0,0,0,0,0,0,0,0]
8
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
8
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
9
[0]
1
[0,0]
1
[0,1,0,1,0]
1
[0,0,1,0,1,0,0]
2
*/
use std::iter;
struct SolutionTop;

impl SolutionTop {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        n == 0
            || flowerbed
                .iter()
                .chain(iter::once(&0))
                .fold((1, 0), |(consecutive_zeros, open_pots), &pot| {
                    if pot == 1 {
                        (0, open_pots)
                    } else if consecutive_zeros == 2 {
                        (1, open_pots + 1)
                    } else {
                        (consecutive_zeros + 1, open_pots)
                    }
                })
                .1
                >= n
    }
}

#[test]
fn test_605() {
    let result = Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1);
    assert_eq!(result, true);

    let result = Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2);
    assert_eq!(result, false);

    let result = Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2);
    assert_eq!(result, false);
}

#[test]
fn test_605_1() {
    let result = Solution1::can_place_flowers(vec![1, 0, 0, 0, 1], 1);
    assert_eq!(result, true);
    let result = Solution1::can_place_flowers(vec![1, 0, 0, 0, 1], 2);
    assert_eq!(result, false);
    let result = Solution1::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2);
    assert_eq!(result, false);
}

#[test]
fn test_605_top() {
    let result = SolutionTop::can_place_flowers(vec![1, 0, 0, 0, 1], 1);
    assert_eq!(result, true);
    let result = SolutionTop::can_place_flowers(vec![1, 0, 0, 0, 1], 2);
    assert_eq!(result, false);
    let result = SolutionTop::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2);
    assert_eq!(result, false);
}
