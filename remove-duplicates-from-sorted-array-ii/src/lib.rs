pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }

    let mut triple_unique_idx = 0;
    for i in 2..nums.len() {
        let discard = nums[i - 2] == nums[i - 1] && nums[i - 1] == nums[i];
        if !discard {
            nums[triple_unique_idx] = nums[i - 2];
            triple_unique_idx += 1;
        }
    }
    let [second_last, last] = *nums.last_chunk::<2>().unwrap();
    nums[triple_unique_idx] = second_last;
    nums[triple_unique_idx + 1] = last;

    let triple_unique_elements = (triple_unique_idx + 2) as i32;
    triple_unique_elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut numbers = vec![1, 2, 2, 2, 2, 3, 3, 4, 4, 4, 4, 4, 4, 4, 5];

        let result = remove_duplicates(&mut numbers);

        let expected_result_numbers = [1, 2, 2, 3, 3, 4, 4, 5];
        let expected_result = expected_result_numbers.len() as i32;

        assert_eq!(result, expected_result);
        assert_eq!(&numbers[..result as usize], &expected_result_numbers);
    }
}
