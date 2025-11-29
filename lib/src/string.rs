use crate::word::*;

#[derive(Debug)]
pub struct StringComposer {
    completed: String,
    current: HangulWordComposer,
}

impl StringComposer {
    pub fn new() -> Self {
        Self {
            completed: String::new(),
            current: HangulWordComposer::new(),
        }
    }

    pub fn push_char(&mut self, c: char) -> Result<(), String> {
        match self.current.push_char(c) {
            WordPushResult::Continue => Ok(()),
            _ => self.handle_invalid_input(c),
        }
    }

    pub fn as_string(&self) -> Result<String, String> {
        let mut result = self.completed.clone();
        let current_string = self.current.as_string()?;
        result.push_str(&current_string);
        Ok(result)
    }

    pub fn pop(&mut self) -> Result<Option<char>, String> {
        match self.current.pop()? {
            Some(c) => Ok(Some(c.get_char())),
            None => match self.completed.pop() {
                Some(c) => Ok(Some(c)),
                None => Ok(None),
            }
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