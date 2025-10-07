pub fn candy(ratings: Vec<i32>) -> i32 {
    let increasing_child_candies = ratings.chunk_by(|r1, r2| r1 < r2).flat_map(|c| 1..=c.len());
    let decreasing_child_candies = ratings
        .chunk_by(|r1, r2| r1 > r2)
        .flat_map(|c| (1..=c.len()).rev());

    increasing_child_candies
        .zip(decreasing_child_candies)
        .map(|(a, b)| a.max(b))
        .sum::<usize>() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let ratings = vec![1, 0, 2];
        assert_eq!(candy(ratings), 5);
    }

    #[test]
    fn it_works2() {
        let ratings = vec![1, 3, 4, 5, 2];
        assert_eq!(candy(ratings), 11);
    }
}
