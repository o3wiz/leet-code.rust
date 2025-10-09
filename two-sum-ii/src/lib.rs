pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    assert!(!numbers.is_empty());

    let mut left_idx = 0;
    let mut right_idx = numbers.len() - 1;
    while left_idx < right_idx {
        let sum = numbers[left_idx] + numbers[right_idx];
        match sum.cmp(&target) {
            std::cmp::Ordering::Less => left_idx += 1,
            std::cmp::Ordering::Greater => right_idx -= 1,
            std::cmp::Ordering::Equal => {
                let left_number = (left_idx + 1) as i32;
                let right_number = (right_idx + 1) as i32;
                return vec![left_number, right_number];
            }
        }
    }

    vec![-1, -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }
}
