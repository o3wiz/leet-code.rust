pub fn can_jump(jumps: Vec<i32>) -> bool {
    let mut jumpable_pos = jumps.len() - 1;

    for (idx, &jump_count) in jumps.iter().enumerate().rev() {
        if idx + (jump_count as usize) >= jumpable_pos {
            jumpable_pos = idx;
        }
    }

    jumpable_pos == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let jumps = vec![2, 3, 1, 1, 4];
        assert!(can_jump(jumps));
    }

    #[test]
    fn second() {
        let jumps = vec![3, 2, 1, 0, 4];
        assert!(!can_jump(jumps));
    }
}
