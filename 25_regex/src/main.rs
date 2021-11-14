#![allow(dead_code)]

use std::char;

fn main() {}

fn is_match<S: AsRef<str>, R: AsRef<str>>(string: S, pattern: R) -> bool {
    let string: Vec<char> = string.as_ref().chars().collect();
    let pattern: Vec<char> = pattern.as_ref().chars().collect();
    if pattern.is_empty() {
        return string.is_empty();
    }

    if pattern.len() == 1 && pattern[0] == '*' {
        return false;
    }

    let mut dp: Vec<Vec<bool>> = (0..string.len() + 1)
        .map(|_| vec![false; pattern.len() + 1])
        .collect();

    dp[0][0] = true;
    for p in 2..=pattern.len() {
        dp[0][p] = pattern[p - 1] == '*' && dp[0][p - 2];
    }

    for p in 1..=pattern.len() {
        for s in 1..=string.len() {
            if pattern[p - 1] == string[s - 1] || pattern[p - 1] == '.' {
                dp[s][p] = dp[s - 1][p - 1];
            } else if pattern[p - 1] == '*' {
                dp[s][p] = dp[s][p - 2]
                    || ((string[s - 1] == pattern[p - 2] || pattern[p - 2] == '.') && dp[s - 1][p]);
            }
        }
    }

    dp[string.len()][pattern.len()]
}

#[test]
fn test_star_pattern() {
    assert_eq!(is_match("aaaba", "a*aba"), true);
    assert_eq!(is_match("aaaba", "ab*"), false);
    assert_eq!(is_match("aaaba", "a*ab*"), false);
    assert_eq!(is_match("aaaba", "a*ab*a"), true);
    assert_eq!(is_match("aaaaaaaaaa", "a*ab*a"), true);
    assert_eq!(is_match("aaaaaaaaabbbbbbbba", "a*ab*a"), true);
    assert_eq!(is_match("aaaaaaabbbbb", ".*"), true);
    assert_eq!(is_match("", ".*"), true);
    assert_eq!(is_match("", "."), false);
    assert_eq!(is_match("", "*"), false);
    assert_eq!(is_match("", ""), true);
}
