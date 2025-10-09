pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let max_width = max_width as usize;

    let mut lines = Vec::new();

    let words_count = words.len();
    let mut curr_word_idx = 0;
    while curr_word_idx < words_count {
        let mut next_word_idx = curr_word_idx + 1;
        let mut min_chars_needed = words[curr_word_idx].len();
        while next_word_idx < words_count {
            min_chars_needed += 1 + words[next_word_idx].len();
            if min_chars_needed > max_width {
                break;
            }
            next_word_idx += 1;
        }

        let line_words = &words[curr_word_idx..next_word_idx];
        let line_words_count = line_words.len();
        let left_justified = line_words_count == 1 || next_word_idx == words_count;

        let mut line = String::with_capacity(max_width);
        if left_justified {
            line.push_str(&line_words.join(" "));
            let left_chars = max_width - line.len();
            line.push_str(&" ".repeat(left_chars));
        } else {
            let spaces = max_width - line_words.iter().map(|word| word.len()).sum::<usize>();
            let right_word_spaces = spaces / (line_words_count - 1);
            let left_words_count = if spaces % (line_words_count - 1) == 0 {
                0
            } else {
                spaces % (line_words_count - 1) + 1
            };

            let line_left_words = &line_words[..left_words_count];
            line.push_str(&line_left_words.join(&" ".repeat(right_word_spaces + 1)));

            if left_words_count != 0 {
                line.push_str(&" ".repeat(right_word_spaces));
            }
            let line_right_words = &line_words[left_words_count..];
            line.push_str(&line_right_words.join(&" ".repeat(right_word_spaces)));
        }

        lines.push(line);

        curr_word_idx = next_word_idx;
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words: Vec<String> = vec![
            String::from("Science"),
            String::from("is"),
            String::from("what"),
            String::from("we"),
            String::from("understand"),
            String::from("well"),
            String::from("enough"),
            String::from("to"),
            String::from("explain"),
            String::from("to"),
            String::from("a"),
            String::from("computer."),
            String::from("Art"),
            String::from("is"),
            String::from("everything"),
            String::from("else"),
            String::from("we"),
            String::from("do"),
        ];
        let max_width = 20;

        assert_eq!(
            full_justify(words, max_width),
            vec![
                String::from("Science  is  what we"),
                String::from("understand      well"),
                String::from("enough to explain to"),
                String::from("a  computer.  Art is"),
                String::from("everything  else  we"),
                String::from("do                  "),
            ]
        );
    }
}
