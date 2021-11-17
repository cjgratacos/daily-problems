fn main() {}

fn water(dimensions: &[usize]) -> usize {
    let mut result = 0;

    let mut left_max: usize = 0;
    let mut right_max: usize = 0;

    let mut low_index: usize = 0;
    let mut hi_index: usize = dimensions.len() - 1;

    loop {
        println!(
            "L:{}:{}, R:{}:{}, {}",
            low_index, left_max, hi_index, right_max, result
        );
        if low_index >= hi_index {
            break;
        }

        if dimensions[low_index] < dimensions[hi_index] {
            if dimensions[low_index] > left_max {
                left_max = dimensions[low_index];
            } else {
                result += left_max - dimensions[low_index];
            }
            low_index += 1;
        } else {
            if dimensions[hi_index] > right_max {
                right_max = dimensions[hi_index];
            } else {
                result += right_max - dimensions[hi_index];
            }

            hi_index -= 1;
        }
    }

    result
}

#[test]
fn test() {
    assert_eq!(water(&[2, 1, 2]) == 1, true);
    assert_eq!(water(&[2, 0, 2]) == 2, true);
    assert_eq!(water(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]) == 6, true);
    assert_eq!(water(&[3, 0, 2, 0, 4]) == 7, true);
    assert_eq!(water(&[3, 0, 1, 3, 0, 5]) == 8, true);
    assert_eq!(water(&[4, 2, 0, 3, 2, 5]) == 9, true);
}
