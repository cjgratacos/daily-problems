fn main() {}

fn median(arr: &[f64]) -> Vec<f64> {
    let mut result = Vec::new();
    let mut median = Vec::new();
    for (i, v) in arr.iter().enumerate() {
        median.push(*v);
        if median.len() > 2 {
            median.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }

        if (i + 1) % 2 == 0 {
            let x = *median.get(median.len() / 2).unwrap();
            let y = *median.get((median.len() / 2) - 1).unwrap();
            result.push((x + y) / 2.0);
        } else {
            result.push(*median.get(median.len() / 2).unwrap());
        }
    }

    result
}

#[test]
fn test() {
    assert_eq!(
        median(&[2.0, 1.0, 5.0, 7.0, 2.0, 0.0, 5.0]).eq(&[2.0, 1.5, 2.0, 3.5, 2.0, 2.0, 2.0]),
        true
    );
}
