struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut vd = std::collections::VecDeque::with_capacity(k as usize);
        let mut max = 0;
        let mut count = 0;
        for c in s.chars() {
            if count == k {
                return k;
            }
            vd.push_back(c);
            if Self::is_vowel(c) {
                count += 1;
            }
            if vd.len() > k as usize {
                let c = vd.pop_front().unwrap();
                if Self::is_vowel(c) {
                    count -= 1;
                }
            }
            max = max.max(count);
        }

        max
    }

    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
}

/// Top Voted Solution
struct SolutionTop;

fn is_vowel(ch: u8) -> bool {
    match ch {
        105 | 111 | 97 | 117 | 101 => true,
        _ => false,
    }
}

impl SolutionTop {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let count: usize = s.iter().take(k).filter(|&x| is_vowel(*x)).count();
        s.iter()
            .skip(k)
            .zip(s.iter())
            .fold((count, count), |a, x| {
                let (max, count) = a;
                let (cur, left) = x;
                match (is_vowel(*left), is_vowel(*cur)) {
                    (false, true) => (max.max(count + 1), count + 1),
                    (true, false) => (max, count - 1),
                    _ => (max, count),
                }
            })
            .0 as i32
    }
}

#[test]
fn test_1456() {
    let s = "abciiidef".to_owned();
    let k = 3;
    let res = 3;
    assert_eq!(Solution::max_vowels(s, k), res);

    let s = "aeiou".to_owned();
    let k = 2;
    let res = 2;
    assert_eq!(Solution::max_vowels(s, k), res);

    let s = "leetcode".to_owned();
    let k = 3;
    let res = 2;
    assert_eq!(Solution::max_vowels(s, k), res);

    let s = "rhythms".to_owned();
    let k = 4;
    let res = 0;
    assert_eq!(Solution::max_vowels(s, k), res);

    let s = "tryhard".to_owned();
    let k = 4;
    let res = 1;
    assert_eq!(Solution::max_vowels(s, k), res);

    let s = "zpuerktejfp".to_owned();
    let k = 3;
    let res = 2;
    assert_eq!(Solution::max_vowels(s, k), res);
}

#[test]
fn test_1456_top() {
    let s = "abciiidef".to_owned();
    let k = 3;
    let res = 3;
    assert_eq!(SolutionTop::max_vowels(s, k), res);
    let s = "aeiou".to_owned();
    let k = 2;
    let res = 2;
    assert_eq!(SolutionTop::max_vowels(s, k), res);
    let s = "leetcode".to_owned();
    let k = 3;
    let res = 2;
    assert_eq!(SolutionTop::max_vowels(s, k), res);
    let s = "rhythms".to_owned();
    let k = 4;
    let res = 0;
    assert_eq!(SolutionTop::max_vowels(s, k), res);
    let s = "tryhard".to_owned();
    let k = 4;
    let res = 1;
    assert_eq!(SolutionTop::max_vowels(s, k), res);
    let s = "zpuerktejfp".to_owned();
    let k = 3;
    let res = 2;
    assert_eq!(SolutionTop::max_vowels(s, k), res);
}
