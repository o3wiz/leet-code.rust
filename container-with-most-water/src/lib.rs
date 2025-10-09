pub fn max_area(heights: Vec<i32>) -> i32 {
    assert!(!heights.is_empty());

    let mut max_volume = 0;

    let mut left_idx = 0;
    let mut right_idx = heights.len() - 1;
    while left_idx < right_idx {
        let height = heights[left_idx].min(heights[right_idx]) as usize;
        let width = right_idx - left_idx;
        let current_volume = height * width;
        max_volume = max_volume.max(current_volume);
        if heights[left_idx] < heights[right_idx] {
            left_idx += 1;
        } else {
            right_idx -= 1;
        }
    }

    max_volume as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let heights =  vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(max_area(heights), 49);
    }
}
