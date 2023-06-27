struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let (mut str1, mut str2) = (str1, str2);
        if str1.len() < str2.len() {
            std::mem::swap(&mut str1, &mut str2);
        }
        if str1.starts_with(&str2) {
            let (_a, b) = str1.split_at(str2.len());
            if b.is_empty() {
                return str2;
            }
            return Self::gcd_of_strings(str2, b.to_string());
        }
        "".to_string()
    }
}

struct Solution1;
impl Solution1 {
    pub fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        Self::gcd(b, a % b)
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let len = Self::gcd(str1.len(), str2.len());
        let combine12 = str1.clone() + &str2;
        let combine21 = str2.clone() + &str1;
        if combine12 != combine21 {
            return "".to_string();
        }

        str1[..len].to_string()
    }
}

/// best voted Solution
struct SolutionTop;
impl SolutionTop {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if &(str1.clone() + &str2) != &(str2.clone() + &str1) {
            return "".to_string();
        }
        let length = gcd(str1.len(), str2.len());
        str1[0..length].to_owned()
    }
}
pub fn gcd(n1: usize, n2: usize) -> usize {
    if n2 == 0 {
        return n1;
    }
    gcd(n2, n1 % n2)
}

#[test]
fn test_1071() {
    let result = Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string());
    assert_eq!(result, "ABC");
    let result = Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string());
    assert_eq!(result, "AB");
    let result = Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string());
    assert_eq!(result, "");
}

#[test]
fn test_1071_1() {
    let result = Solution1::gcd_of_strings("ABCABC".to_string(), "ABC".to_string());
    assert_eq!(result, "ABC");
    let result = Solution1::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string());
    assert_eq!(result, "AB");
    let result = Solution1::gcd_of_strings("LEET".to_string(), "CODE".to_string());
    assert_eq!(result, "");
    let result = Solution1::gcd_of_strings("ABABABAB".to_string(), "ABAB".to_string());
    assert_eq!(result, "ABAB");
}

#[test]
fn test_1071_top() {
    let result = SolutionTop::gcd_of_strings("ABCABC".to_string(), "ABC".to_string());
    assert_eq!(result, "ABC");
    let result = SolutionTop::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string());
    assert_eq!(result, "AB");
    let result = SolutionTop::gcd_of_strings("LEET".to_string(), "CODE".to_string());
    assert_eq!(result, "");
    let result = SolutionTop::gcd_of_strings("ABABABAB".to_string(), "ABAB".to_string());
    assert_eq!(result, "ABAB");
}
