pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    let mut j = 0;
    while i < nums.len() {
        if nums[i] != val {
            nums[j] = nums[i];
            j += 1;
        }
        i += 1;
    }

    j as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut numbers = vec![1, 5, 2, 3, 5, 4, 5, 5];
        println!("{:?}", numbers);
        let result = remove_element(&mut numbers, 5);
        println!("{:?} [{result}]", numbers);
    }
}
