fn main() {}

fn is_balance(s: &str) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        if c == '{' || c == '[' || c == '(' {
            stack.push(c);
        }

        if c == '}' || c == ']' || c == ')' {
            match stack.pop() {
                Some(element) => {
                    if !((element == '{' && c == '}')
                        || (element == '(' && c == ')')
                        || (element == '[' && c == ']'))
                    {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }

    return stack.is_empty();
}

#[test]
fn test() {
    assert_eq!(is_balance("([])[]({})"), true);
    assert_eq!(is_balance("([)]"), false);
    assert_eq!(is_balance("((()"), false);
}
