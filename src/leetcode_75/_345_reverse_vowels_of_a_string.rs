struct Solution;
struct SolutionTop;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if !vowels.contains(&s[i]) {
                i += 1;
                continue;
            }
            if !vowels.contains(&s[j]) {
                j -= 1;
                continue;
            }
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
        s.iter().collect::<String>()
    }
}


/// best voted Solution
impl SolutionTop {
    pub fn reverse_vowels(s: String) -> String {
        let is_vowel = |b: u8| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U');
        let mut vb = s.into_bytes();
        let (mut i, mut j) = (0, vb.len() - 1);
        while i < j {
            while i < j && !is_vowel(vb[i]) {
                i += 1;
            }
            while i < j && !is_vowel(vb[j]) {
                j -= 1;
            }
            if i < j {
                vb.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        String::from_utf8(vb).unwrap()
    }
}

#[test]
fn test_345() {
    assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle".to_string());
    assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
    assert_eq!(Solution::reverse_vowels("aA".to_string()), "Aa".to_string());
}

#[test]
fn test_345_top() {
    assert_eq!(SolutionTop::reverse_vowels("hello".to_string()), "holle".to_string());
    assert_eq!(SolutionTop::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
    assert_eq!(SolutionTop::reverse_vowels("aA".to_string()), "Aa".to_string());

}
