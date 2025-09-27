pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = (k as usize) % nums.len();
    nums.rotate_right(k);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut numbers: Vec<i32> = (1..=7).collect();
        let rotations = 3;
        rotate(&mut numbers, rotations);
        assert_eq!(&numbers, &vec![5, 6, 7, 1, 2, 3, 4]);
    }
}
