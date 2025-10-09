pub fn is_subsequence(s: &str, t: &str) -> bool {
    if s.is_empty() {
        true
    } else if s.len() > t.len() {
        false
    }
    // s is not empty and |s| <= |t| -> 0 < |s| <= |t| -> then |t|, |s| >= 1
    else if s.as_bytes()[0] == t.as_bytes()[0] {
        is_subsequence(&s[1..], &t[1..])
    } else {
        is_subsequence(&s, &t[1..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        assert!(is_subsequence(&s, &t));
    }

    #[test]
    fn it_works2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        assert!(!is_subsequence(&s, &t));
    }
}
