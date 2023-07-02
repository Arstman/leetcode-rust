struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut last_char = chars[0];
        let mut last_char_count = 1;
        let mut write_index = 0;
        for i in 1..chars.len() {
            if chars[i] == last_char {
                last_char_count += 1;
            } else {
                chars[write_index] = last_char;
                write_index += 1;
                if last_char_count > 1 {
                    let count_str = last_char_count.to_string();
                    for c in count_str.chars() {
                        chars[write_index] = c;
                        write_index += 1;
                    }
                }
                last_char = chars[i];
                last_char_count = 1;
            }
        }
        chars[write_index] = last_char;
        write_index += 1;
        if last_char_count > 1 {
            let count_str = last_char_count.to_string();
            for c in count_str.chars() {
                chars[write_index] = c;
                write_index += 1;
            }
        }
        chars.truncate(write_index);
        write_index as i32
    }
}

/// Top Voted Solution
struct SolutionTop;

impl SolutionTop {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut p: usize = 0;
        let mut right: usize = 1;
        let mut counter: usize = 1;

        while right != chars.len() + 1 {
            if right == chars.len() || chars[right] != chars[right - 1] {
                chars[p] = chars[right - 1];

                if counter != 1 {
                    let num_str = counter.to_string();
                    for el in num_str.as_bytes().iter() {
                        p += 1;
                        chars[p] = *el as char;
                    }
                }
                p += 1;
                counter = 0;
            }
            right += 1;
            counter += 1;
        }
        chars.truncate(p);
        return p as i32;
    }
}

#[test]
fn test_443() {
    let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    let res = Solution::compress(&mut chars);
    assert_eq!(res, 6);
    assert_eq!(chars, vec!['a', '2', 'b', '2', 'c', '3']);

    let mut chars = vec!['a'];
    let res = Solution::compress(&mut chars);
    assert_eq!(res, 1);
    assert_eq!(chars, vec!['a']);

    let mut chars = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];
    let res = Solution::compress(&mut chars);
    assert_eq!(res, 4);
    assert_eq!(chars, vec!['a', 'b', '1', '2']);
}

#[test]
fn test_443_top() {
    let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    let res = SolutionTop::compress(&mut chars);
    assert_eq!(res, 6);
    assert_eq!(chars, vec!['a', '2', 'b', '2', 'c', '3']);

    let mut chars = vec!['a'];
    let res = SolutionTop::compress(&mut chars);
    assert_eq!(res, 1);
    assert_eq!(chars, vec!['a']);
   
    let mut chars = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];
    let res = SolutionTop::compress(&mut chars);
    assert_eq!(res, 4);
    assert_eq!(chars, vec!['a', 'b', '1', '2']);
}
