pub fn trap(wall_heights: Vec<i32>) -> i32 {
    let left_max_walls = wall_heights.iter().scan(0, |acc, &height| {
        *acc = height.max(*acc);
        Some(*acc)
    });
    let right_max_walls: Vec<_> = wall_heights
        .iter()
        .rev()
        .scan(0, |acc, &height| {
            *acc = height.max(*acc);
            Some(*acc)
        })
        .collect();
    let min_bounding_wall = left_max_walls
        .zip(right_max_walls.iter().rev())
        .map(|(l_wall, &r_wall)| l_wall.min(r_wall));

    wall_heights
        .iter()
        .zip(min_bounding_wall)
        .map(|(wall, min_bounding_wall)| min_bounding_wall - *wall)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let heights = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(trap(heights), 9);
    }
}
