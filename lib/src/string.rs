use crate::word::*;

/// A composer struct that manages the composition of strings of text
/// consisting of multiple words, including both Hangul words and non-Hangul
/// text.
///
/// The `StringComposer` maintains both a string of completed text and a
/// `HangulWordComposer` for the current word being composed. If the currently
/// active word is a Hangul word that has not yet been completed, pushing or
/// popping characters will interact with the `HangulWordComposer` and directly
/// update syllable blocks. Otherwise, Unicode characters will be directly
/// added to or removed from the completed string.
///
/// **API:**
/// ```rust
/// use hangul::string::StringComposer;
///
/// let mut composer = StringComposer::new();
///
/// // Push characters to form Hangul syllables
/// composer.push_char('ㅎ').unwrap();
/// composer.push_char('ㅏ').unwrap();
///
/// // Get the composed string
/// let result = composer.as_string().unwrap();
/// assert_eq!(result, "하".to_string());
///
/// // Push non-Hangul characters
/// composer.push_char(' ').unwrap();
/// composer.push_char('!').unwrap();
/// assert_eq!(composer.as_string().unwrap(), "하 !".to_string());
///
/// // Popping non-Hangul characters removes them from the completed string
/// composer.pop().unwrap(); // removes '!'
/// composer.pop().unwrap(); // removes ' '
/// assert_eq!(composer.as_string().unwrap(), "하".to_string());
///
/// // Popping Hangul characters after they've been completed removes entire syllables
/// composer.pop().unwrap(); // removes '하'
/// assert_eq!(composer.as_string().unwrap(), "".to_string());
///
/// // Popping characters while a Hangul word is active removes jamo
/// composer.push_char('ㅂ').unwrap();
/// composer.push_char('ㅏ').unwrap();
/// composer.push_char('ㅂ').unwrap();
/// composer.pop().unwrap(); // removes 'ㅂ'
/// assert_eq!(composer.as_string().unwrap(), "바".to_string());
/// ```
#[derive(Debug)]
pub struct StringComposer {
    completed: String,
    current: HangulWordComposer,
}

impl StringComposer {
    /// Creates a new, empty `StringComposer`.
    pub fn new() -> Self {
        Self {
            completed: String::new(),
            current: HangulWordComposer::new(),
        }
    }

    /// Pushes a character to the `StringComposer`.
    ///
    /// If the character is part of a Hangul word, it will be composed into syllables.
    /// Otherwise, it will be added directly to the completed string.
    pub fn push_char(&mut self, c: char) -> Result<(), String> {
        match self.current.push_char(c) {
            WordPushResult::Continue => Ok(()),
            _ => self.handle_invalid_input(c),
        }
    }

    /// Returns the composed string, combining completed text and the current word.
    pub fn as_string(&self) -> Result<String, String> {
        let mut result = self.completed.clone();
        let current_string = self.current.as_string()?;
        result.push_str(&current_string);
        Ok(result)
    }

    /// Pops the last character from the `StringComposer` and returns it wrapped
    /// within a `Result` and `Option`.
    ///
    /// If the current word is a Hangul word with uncompleted syllables, it will
    /// remove the last jamo from the current syllable block. Otherwise, it will
    /// remove the last character from the completed string.
    pub fn pop(&mut self) -> Result<Option<char>, String> {
        match self.current.pop()? {
            Some(c) => Ok(Some(c.get_char())),
            None => match self.completed.pop() {
                Some(c) => Ok(Some(c)),
                None => Ok(None),
            },
        }
    }

    fn handle_invalid_input(&mut self, c: char) -> Result<(), String> {
        let current_string = self.current.as_string()?;
        self.completed.push_str(&current_string);
        self.completed.push(c);
        self.current = HangulWordComposer::new();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_new_words() {
        let input = "ㅎㅏㄴㄱㅡㄹ";
        let mut composer = StringComposer::new();
        for c in input.chars() {
            composer.push_char(c).unwrap();
        }
        let result = composer.as_string().unwrap();
        assert_eq!(result, "한글".to_string());
    }

    #[test]
    fn test_new_hangul_word() {
        let input = "ㅎㅏㄴㄱㅡㄹ ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ";
        let mut composer = StringComposer::new();
        for c in input.chars() {
            composer.push_char(c).unwrap();
        }
        let result = composer.as_string().unwrap();
        assert_eq!(result, "한글 안녕하세요".to_string());
    }

    #[test]
    fn test_new_non_hangul_word() {
        let input = "ㅎㅏㄴㄱㅡㄹ beans";
        let mut composer = StringComposer::new();
        for c in input.chars() {
            composer.push_char(c).unwrap();
        }
        let result = composer.as_string().unwrap();
        assert_eq!(result, "한글 beans".to_string());
    }

    #[test]
    fn test_multiple_words() {
        let input = "ㅎㅏㄴㄱㅡㄹ 123  \n ㅇㅏㄴㄴㅕㅇ!";
        let mut composer = StringComposer::new();
        for c in input.chars() {
            composer.push_char(c).unwrap();
        }
        let result = composer.as_string().unwrap();
        assert_eq!(result, "한글 123  \n 안녕!".to_string());
    }

    #[test]
    fn test_backspace() {
        let input = "ㅇㅏㄴㄴㅕㅇ ㄹㅏㅁㅕㄴ";
        let mut composer = StringComposer::new();
        for c in input.chars() {
            composer.push_char(c).unwrap();
        }
        for _ in 0..7 {
            composer.pop().unwrap();
        }
        let result = composer.as_string().unwrap();
        assert_eq!(result, "안".to_string());
    }
}
