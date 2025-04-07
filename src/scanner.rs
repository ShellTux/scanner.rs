pub struct Scanner<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner { input, position: 0 }
    }

    pub fn next_number(&mut self) -> Option<i32> {
        let remaining = self.get_remaining();

        let (number_len, valid_chars_count): (usize, usize) = remaining
            .trim_start()
            .char_indices()
            .take_while(|&(_, c)| c.is_digit(10) || c == '-')
            .fold((0, 0), |(_, count), (i, _)| (i + 1, count + 1));
        let number_len = number_len + remaining.len() - remaining.trim_start().len();

        if valid_chars_count > 0 {
            self.position += number_len;
            Some(remaining[..number_len].trim_start().parse::<i32>().ok()?)
        } else {
            None
        }
    }

    pub fn next_word(&mut self) -> Option<&'a str> {
        let remaining = self.get_remaining();

        let mut word_len: usize = 0;
        let mut valid_chars_count: usize = 0;

        for (i, c) in remaining.char_indices() {
            if !c.is_whitespace() {
                valid_chars_count += 1;
                word_len = i + 1;
            } else {
                word_len = i;

                if valid_chars_count > 0 {
                    break;
                }
            }
        }

        if valid_chars_count > 0 {
            self.position += word_len;
            Some(remaining[..word_len].trim_start())
        } else {
            None
        }
    }

    pub fn next_line(&mut self) -> Option<&'a str> {
        let remaining = self.get_remaining();

        if let Some(newline_pos) = remaining.find('\n') {
            let line = &remaining[..newline_pos];

            self.position += newline_pos + 1;
            Some(line.trim_end())
        } else if !remaining.is_empty() {
            let line = remaining;
            self.position = self.input.len();
            Some(line.trim_end())
        } else {
            None
        }
    }

    pub fn get_remaining(&self) -> &'a str {
        &self.input[self.position..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut scanner = Scanner::new("");
        assert_eq!(scanner.next_number(), None);
    }

    #[test]
    fn test_next_number_single_digit() {
        let mut scanner = Scanner::new("7 and more");
        assert_eq!(scanner.next_number(), Some(7));
        assert_eq!(scanner.get_remaining(), " and more");
    }

    #[test]
    fn test_next_word() {
        let mut scanner = Scanner::new("Hello world!");
        assert_eq!(scanner.next_word(), Some("Hello"));
        assert_eq!(scanner.get_remaining(), " world!");
        assert_eq!(scanner.next_word(), Some("world!"));
        assert_eq!(scanner.next_word(), None);
    }

    #[test]
    fn test_next_number_negative() {
        let mut scanner = Scanner::new("-3 is less than 0");
        assert_eq!(scanner.next_number(), Some(-3));
        assert_eq!(scanner.get_remaining(), " is less than 0");
    }

    #[test]
    fn test_next_number_multiple_numbers() {
        let mut scanner = Scanner::new("42 7 -8\n-9\n-40\n  -30    \n33   \n  85");
        assert_eq!(scanner.next_number(), Some(42));
        assert_eq!(scanner.next_number(), Some(7));
        assert_eq!(scanner.next_number(), Some(-8));
        assert_eq!(scanner.next_number(), Some(-9));
        assert_eq!(scanner.next_number(), Some(-40));
        assert_eq!(scanner.next_number(), Some(-30));
        assert_eq!(scanner.next_number(), Some(33));
        assert_eq!(scanner.next_number(), Some(85));
        assert_eq!(scanner.next_number(), None);
    }

    #[test]
    fn test_next_number_no_digits() {
        let mut scanner = Scanner::new("no numbers here");
        assert_eq!(scanner.next_number(), None);
        assert_eq!(scanner.get_remaining(), "no numbers here");
    }

    #[test]
    fn test_next_number_with_whitespace() {
        let mut scanner = Scanner::new("   55   88");
        assert_eq!(scanner.next_number(), Some(55));
        assert_eq!(scanner.get_remaining(), "   88");
        assert_eq!(scanner.next_number(), Some(88));
        assert_eq!(scanner.next_number(), None);
    }

    #[test]
    fn test_get_remaining() {
        let mut scanner = Scanner::new("123 456");
        assert_eq!(scanner.get_remaining(), "123 456");
        scanner.next_number();
        assert_eq!(scanner.get_remaining(), " 456");
        scanner.next_number();
        assert_eq!(scanner.get_remaining(), "");
    }

    #[test]
    fn test_next_number_non_digit_characters() {
        let mut scanner = Scanner::new("abc 123 xyz 456");

        assert_eq!(scanner.next_number(), None);
        assert_eq!(scanner.get_remaining(), "abc 123 xyz 456");
        assert_eq!(scanner.next_word(), Some("abc"));
        assert_eq!(scanner.get_remaining(), " 123 xyz 456");
        assert_eq!(scanner.next_word(), Some("123"));
        assert_eq!(scanner.get_remaining(), " xyz 456");
        assert_eq!(scanner.next_number(), None);
        assert_eq!(scanner.get_remaining(), " xyz 456");
        assert_eq!(scanner.next_word(), Some("xyz"));
        assert_eq!(scanner.get_remaining(), " 456");
        assert_eq!(scanner.next_word(), Some("456"));
        assert_eq!(scanner.get_remaining(), "");
    }

    //#[test]
    //fn test_negative_sign_between_numbers() {
    //    let mut scanner = Scanner::new("5-3");
    //
    //    assert_eq!(scanner.next_number(), Some(5));
    //    assert_eq!(scanner.get_remaining(), "-3");
    //    assert_eq!(scanner.next_number(), Some(-3));
    //
    //    let mut scanner = Scanner::new("5 - 3");
    //
    //    assert_eq!(scanner.next_number(), Some(5));
    //    assert_eq!(scanner.get_remaining(), " - 3");
    //    assert_eq!(scanner.next_number(), None);
    //    assert_eq!(scanner.get_remaining(), " - 3");
    //    assert_eq!(scanner.next_word(), Some("-"));
    //    assert_eq!(scanner.get_remaining(), " 3");
    //    assert_eq!(scanner.next_number(), Some(3));
    //    assert_eq!(scanner.get_remaining(), "");
    //}

    #[test]
    fn test_next_line() {
        let mut scanner = Scanner::new("first line\nsecond line\nthird line");
        assert_eq!(scanner.next_line(), Some("first line"));
        assert_eq!(scanner.get_remaining(), "second line\nthird line");

        assert_eq!(scanner.next_line(), Some("second line"));
        assert_eq!(scanner.get_remaining(), "third line");

        assert_eq!(scanner.next_line(), Some("third line"));
        assert_eq!(scanner.get_remaining(), "");

        assert_eq!(scanner.next_line(), None);
    }

    #[test]
    fn test_next_line_no_newlines() {
        let mut scanner = Scanner::new("single line without newline");
        assert_eq!(scanner.next_line(), Some("single line without newline"));
        assert_eq!(scanner.get_remaining(), "");
        assert_eq!(scanner.next_line(), None);
    }

    #[test]
    fn test_next_line_with_trailing_whitespace() {
        let mut scanner = Scanner::new("line one   \nline two   \nline three   ");
        assert_eq!(scanner.next_line(), Some("line one"));
        assert_eq!(scanner.get_remaining(), "line two   \nline three   ");

        assert_eq!(scanner.next_line(), Some("line two"));
        assert_eq!(scanner.get_remaining(), "line three   ");

        assert_eq!(scanner.next_line(), Some("line three"));
        assert_eq!(scanner.get_remaining(), "");

        assert_eq!(scanner.next_line(), None);
    }
}
