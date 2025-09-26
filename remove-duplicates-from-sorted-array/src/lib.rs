pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut unique_idx = 0;

    for i in 1..nums.len() {
        if nums[i - 1] != nums[i] {
            nums[unique_idx] = nums[i - 1];
            unique_idx += 1;
        }
    }
    nums[unique_idx] = *nums.last().unwrap();

    let unique_elements = (unique_idx + 1) as i32;
    unique_elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut numbers = vec![1, 2, 2, 2, 3, 3, 3, 4, 5, 6, 6, 7];
        let unique_elements = remove_duplicates(&mut numbers) as usize;
        assert_eq!(&numbers[..unique_elements], &[1, 2, 3, 4, 5, 6, 7]);
    }
}
