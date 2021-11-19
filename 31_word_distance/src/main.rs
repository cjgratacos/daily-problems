fn main() {}

fn levenshtein_substitution(x: &str, y: &str) -> usize {
    levenshtein_substitution_helper(x, x.len(), y, y.len())
}

fn levenshtein_substitution_helper(x: &str, xs: usize, y: &str, ys: usize) -> usize {
    if xs == 0 {
        return ys;
    }

    if ys == 0 {
        return xs;
    }

    let cost: usize = if x.chars().nth(xs - 1).unwrap() == y.chars().nth(ys - 1).unwrap() {
        0
    } else {
        1
    };

    let deletion = levenshtein_substitution_helper(x, xs - 1, y, ys) + 1;
    let insertation = levenshtein_substitution_helper(x, xs, y, ys - 1) + 1;
    let substituion = levenshtein_substitution_helper(x, xs - 1, y, ys - 1) + cost;

    substituion.min(insertation.min(deletion))
}

#[test]
fn test() {
    assert_eq!(levenshtein_substitution("kitten", "sitting") == 3, true);
    assert_eq!(levenshtein_substitution("car", "tars") == 2, true);
    assert_eq!(levenshtein_substitution("car", "cat") == 1, true);
    assert_eq!(levenshtein_substitution("", "") == 0, true);
    assert_eq!(levenshtein_substitution("test", "") == 4, true);
    assert_eq!(levenshtein_substitution("", "test") == 4, true);
    assert_eq!(levenshtein_substitution("Saturday", "Sunday") == 3, true);
}
