struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(c1), Some(c2)) => {
                    res.push(c1);
                    res.push(c2);
                }
                (Some(c1), None) => {
                    res.push(c1);
                    res.push_str(iter1.as_str());
                    break;
                }
                (None, Some(c2)) => {
                    res.push(c2);
                    res.push_str(iter2.as_str());
                    break;
                }
                (None, None) => break,
            }
        }
        res
   }
}


/// best voted Solution
/// ```rust 
///  impl Solution {
///      pub fn merge_alternately(word1: String, word2: String) -> String {
///          word1
///              .chars()
///              .zip(word2.chars())  //zip is short-circuiting;
///              .flat_map(|(c1, c2)| [c1, c2])
///              .chain(word1.chars().skip(word2.len()))
///              .chain(word2.chars().skip(word1.len()))
///              .collect()
///      }
///  }
/// ```

#[test]
fn test_1768() {
    let word1 = "abc".to_string();
    let word2 = "pqr".to_string();
    let res = "apbqcr".to_string();
    assert_eq!(Solution::merge_alternately(word1, word2), res);

    let word1 = "ab".to_string();
    let word2 = "pqrs".to_string();
    assert_eq!(Solution::merge_alternately(word1, word2), "apbqrs");

    let word1 = "abcd".to_string();
    let word2 = "pq".to_string();
    assert_eq!(Solution::merge_alternately(word1, word2), "apbqcd");

}
