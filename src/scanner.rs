use std::str::FromStr;

/// A `Scanner` is a simple utility for parsing strings, allowing access to words,
/// numbers, and lines from an input string.
///
/// The `Scanner` maintains a position in the input string and provides
/// methods to extract the next number, word, or line from the remaining
/// input.
pub struct Scanner<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Scanner<'a> {
    /// Creates a new `Scanner` for the given input string.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the input to be scanned.
    ///
    /// # Examples
    ///
    /// ```
    /// use scanner::scanner::Scanner;
    /// let scanner = Scanner::new("Hello, world!");
    /// ```
    pub fn new(input: &'a str) -> Self {
        Scanner { input, position: 0 }
    }

    /// Scans for the next token in the input string based on a provided predicate.
    ///
    /// A token is defined as a contiguous sequence of characters that satisfy the
    /// condition defined by the `predicate` closure. This method will consume the
    /// characters matching the predicate from the input string and update the scanner's position.
    ///
    /// # Arguments
    ///
    /// * `predicate`: A closure that takes a character as input and returns a boolean,
    /// determining whether the character should be considered part of the token.
    ///
    /// # Returns
    ///
    /// * `Option<&'a str>` if a valid token is found.
    /// * `None` if no valid token can be found.
    ///
    /// # Examples
    ///
    /// ```
    /// use scanner::scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello, world!");
    /// // Scan for words (non-whitespace characters)
    /// assert_eq!(scanner.next_token(|c, _| !c.is_whitespace()), Some("Hello,"));
    ///
    /// let mut scanner = Scanner::new("123 456 789");
    /// // Scan for numbers (digits)
    /// assert_eq!(scanner.next_token(|c, _| c.is_digit(10)), Some("123"));
    /// ```
    pub fn next_token<F>(&mut self, predicate: F) -> Option<&'a str>
    where
        F: Fn(char, usize) -> bool,
    {
        let remaining = self.get_remaining();

        let mut token_len: usize = 0;
        let mut valid_chars_count: usize = 0;

        for (i, c) in remaining.char_indices() {
            if predicate(c, i) {
                valid_chars_count += 1;
                token_len = i + 1;
            } else {
                if valid_chars_count > 0 {
                    break;
                }
                token_len = i; // update token length to exclude this character
            }
        }

        if valid_chars_count > 0 {
            self.position += token_len;
            Some(remaining[..token_len].trim_start())
        } else {
            None
        }
    }

    /// Scans for the next number in the input string.
    ///
    /// Parses a contiguous sequence of digits, including an optional leading
    /// minus sign for negative numbers. Consumes the number from the input
    /// and updates the scanner's position.
    ///
    /// # Returns
    ///
    /// * `Some(T)` if a valid number is found.
    /// * `None` if no valid number is found.
    ///
    /// # Examples
    ///
    /// ```
    /// use scanner::scanner::Scanner;
    /// let mut scanner = Scanner::new("42 is the answer");
    /// assert_eq!(scanner.next_number(), Some(42));
    /// ```
    pub fn next_number<T>(&mut self) -> Option<T>
    where
        T: FromStr,
    {
        let position = self.position;
        self.next_token(|c, i| c.is_digit(10) || (c == '-' && i == 0))
            .and_then(|token| match token.parse::<T>() {
                Ok(number) => Some(number),
                Err(_) => {
                    self.position = position;
                    None
                }
            })
    }

    /// Scans for the next word in the input string.
    ///
    /// A word is defined as a contiguous sequence of non-whitespace characters.
    /// Consumes the word from the input and updates the scanner's position.
    ///
    /// # Returns
    ///
    /// * `Some(&str)` if a valid word is found.
    /// * `None` if no valid word is found.
    ///
    /// # Examples
    ///
    /// ```
    /// use scanner::scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello, world!");
    /// assert_eq!(scanner.next_word(), Some("Hello,"));
    /// ```
    pub fn next_word(&mut self) -> Option<&'a str> {
        self.next_token(|c, _| !c.is_whitespace())
    }

    /// Scans for the next line from the input string.
    ///
    /// A line is defined as a sequence of characters terminating with a newline
    /// (`\n`). Consumes the line from the input and updates the scanner's
    /// position.
    ///
    /// # Returns
    ///
    /// * `Some(&str)` if a valid line is found.
    /// * `None` if no line is found (i.e., end of input).
    ///
    /// # Examples
    ///
    /// ```
    /// use scanner::scanner::Scanner;
    /// let mut scanner = Scanner::new("First line\nSecond line");
    /// assert_eq!(scanner.next_line(), Some("First line"));
    /// ```
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

    /// Returns the remaining unscanned input as a string slice.
    ///
    /// # Returns
    ///
    /// A string slice of the remaining input.
    ///
    /// # Examples
    ///
    /// ```
    /// use scanner::scanner::Scanner;
    /// let mut scanner = Scanner::new("Hello world!");
    /// scanner.next_word();
    /// assert_eq!(scanner.get_remaining(), " world!");
    /// ```
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
        assert_eq!(scanner.next_number::<i32>(), None);
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
        assert_eq!(scanner.next_number::<i32>(), None);
    }

    #[test]
    fn test_next_number_no_digits() {
        let mut scanner = Scanner::new("no numbers here");
        assert_eq!(scanner.next_number::<i32>(), None);
        assert_eq!(scanner.get_remaining(), "no numbers here");
    }

    #[test]
    fn test_next_number_with_whitespace() {
        let mut scanner = Scanner::new("   55   88");
        assert_eq!(scanner.next_number(), Some(55));
        assert_eq!(scanner.get_remaining(), "   88");
        assert_eq!(scanner.next_number(), Some(88));
        assert_eq!(scanner.next_number::<i32>(), None);
    }

    #[test]
    fn test_get_remaining() {
        let mut scanner = Scanner::new("123 456");
        assert_eq!(scanner.get_remaining(), "123 456");
        scanner.next_number::<i32>();
        assert_eq!(scanner.get_remaining(), " 456");
        scanner.next_number::<i32>();
        assert_eq!(scanner.get_remaining(), "");
    }

    #[test]
    fn test_next_number_non_digit_characters() {
        let mut scanner = Scanner::new("abc 123 xyz 456");

        assert_eq!(scanner.next_number::<i32>(), None);
        assert_eq!(scanner.get_remaining(), "abc 123 xyz 456");
        assert_eq!(scanner.next_word(), Some("abc"));
        assert_eq!(scanner.get_remaining(), " 123 xyz 456");
        assert_eq!(scanner.next_word(), Some("123"));
        assert_eq!(scanner.get_remaining(), " xyz 456");
        assert_eq!(scanner.next_number::<i32>(), None);
        assert_eq!(scanner.get_remaining(), " xyz 456");
        assert_eq!(scanner.next_word(), Some("xyz"));
        assert_eq!(scanner.get_remaining(), " 456");
        assert_eq!(scanner.next_word(), Some("456"));
        assert_eq!(scanner.get_remaining(), "");
    }

    #[test]
    fn test_negative_sign_between_numbers() {
        let mut scanner = Scanner::new("5-3");

        assert_eq!(scanner.next_number(), Some(5));
        assert_eq!(scanner.get_remaining(), "-3");
        assert_eq!(scanner.next_number(), Some(-3));

        let mut scanner = Scanner::new("5 - 3");

        assert_eq!(scanner.next_number(), Some(5));
        assert_eq!(scanner.get_remaining(), " - 3");
        assert_eq!(scanner.next_number::<i32>(), None);
        assert_eq!(scanner.get_remaining(), " - 3");
        assert_eq!(scanner.next_word(), Some("-"));
        assert_eq!(scanner.get_remaining(), " 3");
        assert_eq!(scanner.next_number(), Some(3));
        assert_eq!(scanner.get_remaining(), "");
    }

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
