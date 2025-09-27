pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut ret: i32 = 0;

    let mut min_price = i32::MAX;
    for price in prices {
        min_price = min_price.min(price);
        let current_profit = price - min_price;
        ret = ret.max(current_profit);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }
}
