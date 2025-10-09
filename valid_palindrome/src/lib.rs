pub fn is_palindrome(s: String) -> bool {
    let clean_byte = |b: &u8| {
        if b.is_ascii_alphanumeric() {
            Some(b.to_ascii_lowercase())
        } else {
            None
        }
    };
    let forward_clean_bytes = s.as_bytes().iter().flat_map(clean_byte);
    let backward_clean_bytes = s.as_bytes().iter().rev().flat_map(clean_byte);
    forward_clean_bytes.eq(backward_clean_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let phrase = String::from("A man, a plan, a canal: Panama");
        assert!(is_palindrome(phrase));
    }

    #[test]
    fn it_works2() {
        let phrase = String::from("race a car");
        assert!(!is_palindrome(phrase));
    }
}
