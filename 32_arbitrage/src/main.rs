fn main() {}

fn has_arbitrage(currencies: &Vec<Vec<f64>>) -> bool {
    let size = currencies.len();

    let mut memo: Vec<Vec<f64>> = currencies
        .iter()
        .map(|currency| currency.iter().map(|it| -it.log2()).collect())
        .collect();

    let mut ans = false;

    for k in 0..size {
        for i in 0..size {
            for j in 0..size {
                memo[i][j] = memo[i][j].min(memo[i][k] + memo[k][j]);
                if memo[i][i] < 0.0 {
                    ans = true;
                }
            }
        }
    }

    return ans;
}

#[test]
fn test() {
    assert_eq!(has_arbitrage(&vec![]), false);
    assert_eq!(has_arbitrage(&vec![vec![1.0]]), false);
    assert_eq!(has_arbitrage(&vec![vec![1.0, 2.0], vec![0.5, 1.0]]), false);
    assert_eq!(has_arbitrage(&vec![vec![1.0, 2.0], vec![0.6, 1.0]]), true);
}
