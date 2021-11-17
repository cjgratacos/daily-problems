fn main() {}

fn encode(s: &str) -> Option<String> {
    if s.is_empty() {
        return None;
    }

    if s.len() == 1 {
        let s = format!("1{}", s.chars().next().unwrap());
        return Some(s);
    }

    let mut encoded = String::new();
    let mut counter = 0;
    let mut last_character = None;
    for (i, c) in s.chars().enumerate() {
        match &last_character {
            Some(lc) => {
                if c == *lc {
                    counter += 1;
                } else {
                    // Setup Previous
                    encoded.push_str(&format!("{}{}", counter, lc));
                    // Reset
                    last_character = Some(c);
                    counter = 1;
                }
                // Handle if last

                if i == s.len() - 1 {
                    encoded.push_str(&format!("{}{}", counter, c));
                }
            }
            // First char
            None => {
                last_character = Some(c);
                counter += 1;
            }
        }
    }
    Some(encoded)
}

#[test]
fn test() {
    let encoded1 = encode("AAAABBBCCDAA");
    assert_eq!(encoded1.is_some(), true);
    assert_eq!(encoded1.unwrap().eq("4A3B2C1D2A"), true);
}
