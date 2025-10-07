pub fn reverse_words(s: String) -> String {
    let words_reversed: Vec<&str> = s.split_whitespace().rev().collect();
    words_reversed.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("      my     name is daniel    ");
        assert_eq!(reverse_words(s), String::from("daniel is name my"));
    }
}
