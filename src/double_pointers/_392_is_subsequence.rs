struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_iter = s.chars();
        let mut t_iter = t.chars();
        while let Some(s_char) = s_iter.next() {
            if let Some(t_char) = t_iter.find(|&x| x == s_char) {
                if s_char != t_char {
                    return false;
                }
            } else {
                return false;
            }
        }
       return true; 
    }
}

/// Top Voted Solution
struct SolutionTop;
impl SolutionTop {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = t.chars();
        for c in s.chars() {
          match iter.find(|&p| p == c) {
            Some(_) => (),
            None => return false
          }
        }
        true
    }
}

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

#[test]
fn test_392_top() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    assert_eq!(SolutionTop::is_subsequence(s, t), true);
    let s = String::from("axc");
    let t = String::from("ahbgdc");
    assert_eq!(SolutionTop::is_subsequence(s, t), false);
    
    let s = String::from("acb");
    let t = String::from("ahbgdc");
    assert_eq!(SolutionTop::is_subsequence(s, t), false);
}

