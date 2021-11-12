#![allow(dead_code)]

fn main() {}

fn is_match(string: &str, pattern: &str) -> bool {
    let mut memo: Vec<Vec<bool>> = (0..string.len() + 1)
        .map(|_| Vec::with_capacity(pattern.len() + 1))
        .collect();
    dp(0, 0, string, pattern, &mut memo)
}

fn dp(i: usize, j: usize, s: &str, p: &str, memo: &mut Vec<Vec<bool>>) -> bool {
    if let Some(found) = memo[i].get(j) {
        return *found;
    }

    todo!()
}

#[test]
fn test_star_pattern() {
    assert_eq!(is_match("aaaba", "*aba"), true);
    assert_eq!(is_match("aaaba", "ab*"), true);
    assert_eq!(is_match("aaaba", "*ab*"), true);
    assert_eq!(is_match("aaaba", "*ab"), false);
}
