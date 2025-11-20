#[derive(Debug, PartialEq, Eq)]
pub enum Letter {
    NonHangul(char),
    Hangul(HangulLetter),
}

#[derive(Debug, PartialEq, Eq)]
pub enum HangulLetter {
    Consonant(char),
    Vowel(char),
}

#[derive(Debug, PartialEq, Eq)]
pub struct HangulBlock {
    pub initial: char,
    pub vowel: char,
    pub final_optional: Option<char>,
}
