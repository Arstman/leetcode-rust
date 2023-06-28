struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
            .trim()
            .to_string()
    }
}

/// Top Voted Solution
struct SolutionTop;

impl SolutionTop {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .rev()
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[test]
fn test_151() {
    let s = "the sky is blue".to_string();
    let res = "blue is sky the".to_string();
    assert_eq!(Solution::reverse_words(s), res);

    let s = "  hello world!  ".to_string();
    let res = "world! hello".to_string();
    assert_eq!(Solution::reverse_words(s), res);

    let s = "a good   example".to_string();
    let res = "example good a".to_string();
    assert_eq!(Solution::reverse_words(s), res);
}


#[test]
fn test_151_top() {
    let s = "the sky is blue".to_string();
    let res = "blue is sky the".to_string();
    assert_eq!(SolutionTop::reverse_words(s), res);

    let s = "  hello world!  ".to_string();
    let res = "world! hello".to_string();
    assert_eq!(SolutionTop::reverse_words(s), res);

    let s = "a good   example".to_string();
    let res = "example good a".to_string();
    assert_eq!(SolutionTop::reverse_words(s), res);

}
