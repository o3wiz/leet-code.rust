pub fn jump(jumps: Vec<i32>) -> i32 {
    let mut jumps_count = 0;

    let mut pos = 0;
    let jumps_size = jumps.len();
    while pos + 1 < jumps_size {
        jumps_count += 1;

        let current_jumps = jumps[pos] as usize;
        let current_pos_furthest_jump = pos + current_jumps;
        if current_pos_furthest_jump + 1 >= jumps_size {
            break;
        }

        for (next_pos, &next_jump) in jumps
            .iter()
            .enumerate()
            .skip(pos + 1)
            .take(current_jumps)
        {
            let current_pos_furthest_jump = pos + (jumps[pos] as usize);
            let next_pos_furthest_jump = next_pos + (next_jump as usize);
            if current_pos_furthest_jump < next_pos_furthest_jump {
                pos = next_pos;
            }
        }
    }

    jumps_count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let numbers = vec![2, 3, 1, 1, 4];
        let result = jump(numbers);
        println!("{result}");
    }
}
