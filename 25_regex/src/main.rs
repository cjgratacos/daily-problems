fn main() {
    println!("Hello, world!");
}

fn is_match(text: &str, pattern: &str) -> bool {
    let mut memo: Vec<Vec<bool>> = Vec::new();
    dp(text, pattern, &mut memo)
}

fn dp(text: &str, pattern: &str, memo: &mut Vec<Vec<bool>>) -> bool {}
