#[warn(dead_code)]
#[warn(unused_variables)]


struct Solution;

impl Solution {
    pub fn find_substring(_s: String, _words: Vec<String>) -> Vec<i32> {
        // try sice.windows with `scan` method
        unimplemented!();
    }
}

/// best voted Solution
/// ```rust

/// ```

#[test]
fn test_30() {
    assert_eq!(
        Solution::find_substring(
            "barfoothefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string()]
        ),
        vec![0, 9]
    );
    assert_eq!(
        Solution::find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "word".to_string()
            ]
        ),
        vec![]
    );
    assert_eq!(
        Solution::find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "good".to_string()
            ]
        ),
        vec![8]
    );
    assert_eq!(
        Solution::find_substring(
            "barfoofoobarthefoobarman".to_string(),
            vec!["bar".to_string(), "foo".to_string(), "the".to_string()]
        ),
        vec![6, 9, 12]
    );
}

#[test]
fn test_30_1() {
    let s = "barfoothefoobarman".to_string();
    assert_eq!(s.as_bytes().windows(2).next().unwrap(), b"ba");
    assert_eq!(s.as_bytes().windows(3).next().unwrap(), b"bar");
    let words = vec!["foo".to_string(), "bar".to_string()];
    let words_bytes: Vec<&[u8]> = words.iter().map(String::as_bytes).collect();
    let word_len = words[0].len();
    assert_eq!(words[0].as_bytes().windows(2).next().unwrap(), b"fo");
    assert_eq!(words[0].as_bytes(), &[102, 111, 111]);
    assert_eq!(
        String::from_utf8(words[0].as_bytes().to_vec()),
        Ok("foo".to_string())
    );
    let foo = s.as_bytes().windows(word_len).nth(3).unwrap();
    assert_eq!(
        words_bytes[0],
        s.as_bytes().windows(word_len).nth(3).unwrap()
    );
    assert_eq!(words_bytes.contains(&foo), true);
}

#[test]
fn test_30_2() {
    let words = vec!["foo".to_string(), "bar".to_string()];
    let mut map = std::collections::HashMap::new();
    for word in words.iter() {
        *map.entry(word.as_bytes()).or_insert(0) += 1;
    }
    let foo = words[0].as_bytes();

    assert_eq!(map.contains_key(foo), true);
}
