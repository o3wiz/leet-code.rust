pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums.clone();
    }

    let mut left_mul = Vec::<i32>::with_capacity(nums.len());
    let mut mul = 1;
    for &num in &nums {
        left_mul.push(mul);
        mul *= num;
    }

    let mut right_mul_reversed = Vec::<i32>::with_capacity(nums.len());
    let mut mul = 1;
    for &num in nums.iter().rev() {
        right_mul_reversed.push(mul);
        mul *= num;
    }

    left_mul
        .iter()
        .zip(right_mul_reversed.iter().rev())
        .map(|(&a, &b)| a * b)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(product_except_self(nums), vec![24, 12, 8, 6]);
    }
}
