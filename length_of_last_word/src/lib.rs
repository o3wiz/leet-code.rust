pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("   fly me   to   the moon  ");
        assert_eq!(length_of_last_word(s), 4); // moon
    }
}
