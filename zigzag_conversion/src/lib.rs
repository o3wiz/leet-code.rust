pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let num_rows = num_rows as usize;
    let mut char_rows: Vec<Vec<char>> = vec![Vec::new(); num_rows];

    let last_row_idx = num_rows - 1;
    let rows_zigzag_indices = (0..=last_row_idx).chain((1..last_row_idx).rev());
    for (row_idx, char) in rows_zigzag_indices.cycle().zip(s.chars()) {
        char_rows[row_idx].push(char);
    }

    char_rows.iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 3;
        assert_eq!(convert(s, num_rows), String::from("PAHNAPLSIIGYIR"));
    }
}
