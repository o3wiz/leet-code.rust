pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let shortest_string_length = strs.iter().min_by_key(|s| s.len()).unwrap().len();
    let mut common_length = 0;
    for byte_idx in 0..shortest_string_length {
        if !strs
            .iter()
            .map(|s| s.as_bytes()[byte_idx])
            .all(|b| b == strs.first().unwrap().as_bytes()[byte_idx])
        {
            break;
        }

        common_length += 1;
    }

    strs.first().unwrap().as_str()[..common_length].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
