fn main() {
    println!("Hello, world!");
}

fn justify(s: &[&str], k: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut line = Vec::new();
    let mut line_wc = 0;
    for (index, entry) in s.iter().enumerate() {
        // Should Add
        let mut added = false;
        if (line_wc + entry.len()) + line.len() <= k {
            added = true;
            line.push(entry.to_string());
            line_wc += entry.len();
        }

        // Calculate & reset & Recreate
        if !added || index == s.len() - 1 {
            println!("{:?}", line);
            let space_location = line.len() - 1;

            // Single element, pad right to left
            if space_location == 0 {
                let mut l = line.join(" ");
                l.push_str(&" ".repeat(k - line_wc));
                result.push(l);
            }

            if space_location > 0 {
                if index == s.len() - 1 && added {
                    let mut l = line.join(" ");
                    l.push_str(&" ".repeat(k - l.len()));
                    result.push(l);
                } else {
                    let base_space = (k - line_wc) / space_location;
                    let mut additional_space = (k - line_wc) % space_location;
                    let mut l = String::new();

                    for (i, w) in line.iter().enumerate() {
                        if i == line.len() - 1 {
                            l.push_str(w);
                        } else {
                            l.push_str(&format!("{}{}", w, " ".repeat(base_space)));
                        }

                        if additional_space > 0 {
                            l.push_str(" ");
                            additional_space -= 1;
                        }
                    }

                    result.push(l);
                }
            }
            line_wc = entry.len();
            line = Vec::new();
            line.push(entry.to_string());

            if index == s.len() - 1 && !added {
                let mut l = line.join(" ");
                l.push_str(&" ".repeat(k - line_wc));
                result.push(l);
            }
        }
    }

    result
}

#[test]
fn test() {
    assert_eq!(
        justify(
            &[
                "This",
                "is",
                "an",
                "example",
                "of",
                "text",
                "justification."
            ],
            16
        )
        .eq(&["This    is    an", "example  of text", "justification.  "]),
        true
    );

    assert_eq!(
        justify(&["What", "must", "be", "acknowledgment", "shall", "be"], 16).eq(&[
            "What   must   be",
            "acknowledgment  ",
            "shall be        "
        ]),
        true
    );

    assert_eq!(
        justify(
            &[
                "Science",
                "is",
                "what",
                "we",
                "understand",
                "well",
                "enough",
                "to",
                "explain",
                "to",
                "a",
                "computer.",
                "Art",
                "is",
                "everything",
                "else",
                "we",
                "do"
            ],
            20
        )
        .eq(&[
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
        ]),
        true
    );
}
