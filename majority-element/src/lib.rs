pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut cand = *nums.first().unwrap();
    let mut frequency = 1;

    for &num in nums.iter().skip(1) {
        if num == cand {
            frequency += 1;
        } else if frequency == 1 {
            cand = num;
        } else {
            frequency -= 1;
        }
    }

    cand
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let numbers = vec![3, 2, 3];
        let m_element = majority_element(numbers);
        assert_eq!(m_element, 3);
    }

    #[test]
    fn second() {
        let numbers = vec![2, 2, 1, 1, 1, 2, 2];
        let m_element = majority_element(numbers);
        assert_eq!(m_element, 2);
    }
}
