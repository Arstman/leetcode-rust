struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars();
        let mut t_chars = t.chars();

        loop {
            let s_char = s_chars.next();
            let t_char = t_chars.next();
            if s_char.is_none() {
                return true;
            }
            if t_char.is_none() {
                return false;
            }
            if s_char == t_char {
                continue;
            } else {
                t_chars = t.chars();
            }
        }
    }
}

/// Top Voted Solution
// struct SolutionTop;

// impl SolutionTop {

// }

#[test]
fn test_392() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    assert_eq!(Solution::is_subsequence(s, t), true);

    let s = String::from("axc");
    let t = String::from("ahbgdc");
    assert_eq!(Solution::is_subsequence(s, t), false);

    let s = String::from("acb");
    let t = String::from("ahbgdc");
    assert_eq!(Solution::is_subsequence(s, t), false);
}

// #[test]
// fn test_X_top() {

// }
