pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .chunk_by(|&a, &b| a <= b)
        .map(|p| p.last().unwrap() - p.first().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let profit_result = max_profit(prices);
        const EXPECTED_RESULT: i32 = 7;
        assert_eq!(profit_result, EXPECTED_RESULT);
    }
}
