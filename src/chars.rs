use std::fmt::Debug;

/// Utilities and types for chars and char operations

// Jamo sets
const CONSONANTS: &str = "ㅂㅈㄷㄱㅅㅁㄴㅇㄹㅎㅋㅌㅊㅍ";
const COMPOSITE_CONSONANTS: &str = "ㄲㄸㅃㅆㅉㄵㄺㅄㄳㄶㄻㄼㄽㄾㄿㅀ";
const INITIAL_COMPOSITE_CONSONANTS: &str = "ㄲㄸㅃㅆㅉ";
const FINAL_COMPOSITE_CONSONANTS: &str = "ㄲㄵㄺㅄㅆㄳㄶㄻㄼㄽㄾㄿㅀ";
const VOWELS: &str = "ㅛㅕㅑㅐㅔㅒㅖㅗㅓㅏㅣㅠㅜㅡ";
const COMPOSITE_VOWELS: &str = "ㅘㅙㅚㅝㅞㅟㅢ";

// Jamo arithmetic
const S_BASE: u32 = 0xAC00;
const L_BASE: u32 = 0x1100;
const V_BASE: u32 = 0x1161;
const T_BASE: u32 = 0x11A7;
const L_COUNT: u32 = 19;
const V_COUNT: u32 = 21;
const T_COUNT: u32 = 28;
const N_COUNT: u32 = V_COUNT * T_COUNT;

pub(crate) fn consonant_doubles(c1: char, c2: char) -> Option<char> {
    match (c1, c2) {
        ('ㄱ', 'ㄱ') => Some('ㄲ'),
        ('ㄷ', 'ㄷ') => Some('ㄸ'),
        ('ㅂ', 'ㅂ') => Some('ㅃ'),
        ('ㅅ', 'ㅅ') => Some('ㅆ'),
        ('ㅈ', 'ㅈ') => Some('ㅉ'),
        _ => None,
    }
}

pub(crate) fn composite_final(c1: char, c2: char) -> Option<char> {
    match (c1, c2) {
        ('ㄱ', 'ㄱ') => Some('ㄲ'),
        ('ㄴ', 'ㅈ') => Some('ㄵ'),
        ('ㄹ', 'ㄱ') => Some('ㄺ'),
        ('ㅂ', 'ㅅ') => Some('ㅄ'),
        ('ㅅ', 'ㅅ') => Some('ㅆ'),
        ('ㄱ', 'ㅅ') => Some('ㄳ'),
        ('ㄴ', 'ㅎ') => Some('ㄶ'),
        ('ㄹ', 'ㅁ') => Some('ㄻ'),
        ('ㄹ', 'ㅂ') => Some('ㄼ'),
        ('ㄹ', 'ㅅ') => Some('ㄽ'),
        ('ㄹ', 'ㅌ') => Some('ㄾ'),
        ('ㄹ', 'ㅍ') => Some('ㄿ'),
        ('ㄹ', 'ㅎ') => Some('ㅀ'),
        _ => None,
    }
}

pub(crate) fn composite_vowel(v1: char, v2: char) -> Option<char> {
    match (v1, v2) {
        ('ㅗ', 'ㅏ') => Some('ㅘ'),
        ('ㅗ', 'ㅐ') => Some('ㅙ'),
        ('ㅗ', 'ㅣ') => Some('ㅚ'),
        ('ㅜ', 'ㅓ') => Some('ㅝ'),
        ('ㅜ', 'ㅔ') => Some('ㅞ'),
        ('ㅜ', 'ㅣ') => Some('ㅟ'),
        ('ㅡ', 'ㅣ') => Some('ㅢ'),
        _ => None,
    }
}

/// Determines the type of Hangul letter for a given character.
/// Does not work for archaic or non-standard jamo like ᅀ.
pub(crate) fn determine_hangul(c: char) -> Letter {
    return if CONSONANTS.contains(c) {
        Letter::Hangul(HangulLetter::Consonant(c))
    } else if VOWELS.contains(c) {
        Letter::Hangul(HangulLetter::Vowel(c))
    } else if COMPOSITE_CONSONANTS.contains(c) {
        Letter::Hangul(HangulLetter::CompositeConsonant(c))
    } else if COMPOSITE_VOWELS.contains(c) {
        Letter::Hangul(HangulLetter::CompositeVowel(c))
    } else {
        Letter::NonHangul(c)
    };
}

pub(crate) fn is_valid_double_initial(c: char) -> bool {
    INITIAL_COMPOSITE_CONSONANTS.contains(c)
}

pub(crate) fn is_valid_composite_final(c: char) -> bool {
    FINAL_COMPOSITE_CONSONANTS.contains(c)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Letter {
    NonHangul(char),
    Hangul(HangulLetter),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HangulLetter {
    Consonant(char),
    CompositeConsonant(char),
    Vowel(char),
    CompositeVowel(char),
}

impl HangulLetter {
    pub(crate) fn get_char(&self) -> char {
        match self {
            HangulLetter::Consonant(c)
            | HangulLetter::CompositeConsonant(c)
            | HangulLetter::Vowel(c)
            | HangulLetter::CompositeVowel(c) => *c,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct HangulBlock {
    pub initial: char,
    pub vowel: char,
    pub final_optional: Option<char>,
}

impl HangulBlock {
    // Extracts the composed Hangul syllable character from the block struct
    pub fn to_char(&self) -> char {
        // Get u32 representation of chars
        let initial_num = self.initial as u32;
        let vowel_num = self.vowel as u32;
        let final_num = match self.final_optional {
            Some(c) => c as u32,
            None => 0,
        };

        // Calculate indices
        let l_index = initial_num - L_BASE;
        let v_index = vowel_num - V_BASE;
        let t_index = if final_num == 0 {
            0
        } else {
            final_num - T_BASE
        };
        let s_index = (l_index * N_COUNT) + (v_index * T_COUNT) + t_index;

        // Unwrapping because this should only ever be called with valid Hangul
        std::char::from_u32(S_BASE + s_index).unwrap()
    }
}

impl Debug for HangulBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char().to_string())
    }
}

pub(crate) fn hangul_blocks_vec_to_string(blocks: &Vec<HangulBlock>) -> String {
    blocks.iter().map(|b| b.to_char()).collect()
}
