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

pub(crate) fn decompose_composite_vowel(c: char) -> Option<(char, char)> {
    match c {
        'ㅘ' => Some(('ㅗ', 'ㅏ')),
        'ㅙ' => Some(('ㅗ', 'ㅐ')),
        'ㅚ' => Some(('ㅗ', 'ㅣ')),
        'ㅝ' => Some(('ㅜ', 'ㅓ')),
        'ㅞ' => Some(('ㅜ', 'ㅔ')),
        'ㅟ' => Some(('ㅜ', 'ㅣ')),
        'ㅢ' => Some(('ㅡ', 'ㅣ')),
        _ => None,
    }
}

/// Determines the type of Hangul letter for a given character.
/// Does not work for archaic or non-standard jamo like ᅀ.
/// Classifies a character as Hangul jamo or non-Hangul.
pub fn determine_hangul(c: char) -> Letter {
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

#[derive(Debug, PartialEq, Eq)]
pub struct HangulBlock {
    pub initial: char,
    pub vowel: char,
    pub final_optional: Option<char>,
}
impl HangulBlock {
    // Extracts the composed Hangul syllable character from the block struct.
    // Assumes all chars are valid jamo.
    pub fn to_char(&self) -> Result<char, u32> {
        // Ensure the initial, vowel, and final are modern Jamo and not
        // compatibility jamo
        let initial = modernize_jamo_initial(self.initial);
        let vowel = modernize_jamo_vowel(self.vowel);
        let final_optional = match self.final_optional {
            Some(c) => Some(modernize_jamo_final(c)),
            None => None,
        };

        // Get u32 representation of chars
        let initial_num = initial as u32;
        let vowel_num = vowel as u32;
        let final_num = match final_optional {
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
        if let Some(c) = std::char::from_u32(S_BASE + s_index) {
            Ok(c)
        } else {
            Err(S_BASE + s_index)
        }
    }
}

fn modernize_jamo_initial(c: char) -> char {
    match c {
        '\u{3131}' => '\u{1100}', // ㄱ
        '\u{3132}' => '\u{1101}', // ㄲ
        '\u{3134}' => '\u{1102}', // ㄴ
        '\u{3137}' => '\u{1103}', // ㄷ
        '\u{3138}' => '\u{1104}', // ㄸ
        '\u{3139}' => '\u{1105}', // ㄹ
        '\u{3141}' => '\u{1106}', // ㅁ
        '\u{3142}' => '\u{1107}', // ㅂ
        '\u{3143}' => '\u{1108}', // ㅃ
        '\u{3145}' => '\u{1109}', // ㅅ
        '\u{3146}' => '\u{110A}', // ㅆ
        '\u{3147}' => '\u{110B}', // ㅇ
        '\u{3148}' => '\u{110C}', // ㅈ
        '\u{3149}' => '\u{110D}', // ㅉ
        '\u{314A}' => '\u{110E}', // ㅊ
        '\u{314B}' => '\u{110F}', // ㅋ
        '\u{314C}' => '\u{1110}', // ㅌ
        '\u{314D}' => '\u{1111}', // ㅍ
        '\u{314E}' => '\u{1112}', // ㅎ
        other => other,
    }
}

fn modernize_jamo_vowel(c: char) -> char {
    match c {
        '\u{314F}' => '\u{1161}', // ㅏ
        '\u{3150}' => '\u{1162}', // ㅐ
        '\u{3151}' => '\u{1163}', // ㅑ
        '\u{3152}' => '\u{1164}', // ㅒ
        '\u{3153}' => '\u{1165}', // ㅓ
        '\u{3154}' => '\u{1166}', // ㅔ
        '\u{3155}' => '\u{1167}', // ㅕ
        '\u{3156}' => '\u{1168}', // ㅖ
        '\u{3157}' => '\u{1169}', // ㅗ
        '\u{3158}' => '\u{116A}', // ㅘ
        '\u{3159}' => '\u{116B}', // ㅙ
        '\u{315A}' => '\u{116C}', // ㅚ
        '\u{315B}' => '\u{116D}', // ㅛ
        '\u{315C}' => '\u{116E}', // ㅜ
        '\u{315D}' => '\u{116F}', // ㅝ
        '\u{315E}' => '\u{1170}', // ㅞ
        '\u{315F}' => '\u{1171}', // ㅟ
        '\u{3160}' => '\u{1172}', // ㅠ
        '\u{3161}' => '\u{1173}', // ㅡ
        '\u{3162}' => '\u{1174}', // ㅢ
        '\u{3163}' => '\u{1175}', // ㅣ
        other => other,
    }
}

fn modernize_jamo_final(c: char) -> char {
    match c {
        '\u{3131}' => '\u{11A8}', // ㄱ
        '\u{3132}' => '\u{11A9}', // ㄲ
        '\u{3133}' => '\u{11AA}', // ㄳ
        '\u{3134}' => '\u{11AB}', // ㄴ
        '\u{3135}' => '\u{11AC}', // ㄵ
        '\u{3136}' => '\u{11AD}', // ㄶ
        '\u{3137}' => '\u{11AE}', // ㄷ
        '\u{3139}' => '\u{11AF}', // ㄹ
        '\u{313A}' => '\u{11B0}', // ㄺ
        '\u{313B}' => '\u{11B1}', // ㄻ
        '\u{313C}' => '\u{11B2}', // ㄼ
        '\u{313D}' => '\u{11B3}', // ㄽ
        '\u{313E}' => '\u{11B4}', // ㄾ
        '\u{313F}' => '\u{11B5}', // ㄿ
        '\u{3140}' => '\u{11B6}', // ㅀ
        '\u{3141}' => '\u{11B7}', // ㅁ
        '\u{3142}' => '\u{11B8}', // ㅂ
        '\u{3144}' => '\u{11B9}', // ㅄ
        '\u{3145}' => '\u{11BA}', // ㅅ
        '\u{3146}' => '\u{11BB}', // ㅆ
        '\u{3147}' => '\u{11BC}', // ㅇ
        '\u{3148}' => '\u{11BD}', // ㅈ
        '\u{314A}' => '\u{11BE}', // ㅊ
        '\u{314B}' => '\u{11BF}', // ㅋ
        '\u{314C}' => '\u{11C0}', // ㅌ
        '\u{314D}' => '\u{11C1}', // ㅍ
        '\u{314E}' => '\u{11C2}', // ㅎ
        other => other,
    }
}

// Convert compatibility jamo to modern jamo.

pub(crate) fn hangul_blocks_vec_to_string(blocks: &Vec<HangulBlock>) -> Result<String, String> {
    let mut result = String::new();
    for block in blocks {
        match block.to_char() {
            Ok(c) => result.push(c),
            Err(codepoint) => {
                return Err(format!(
                    "Failed to convert HangulBlock: {:?} to char. Invalid codepoint: U+{:X}",
                    block, codepoint
                ));
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determine_hangul_identifies_valid_consonants() {
        let consonants = "ㅂㅈㄷㄱㅅㅁㄴㅇㄹㅎㅋㅌㅊㅍ";
        for c in consonants.chars() {
            let result = determine_hangul(c);
            assert!(
                result == Letter::Hangul(HangulLetter::Consonant(c)),
                "Failed on consonant: {}; got result: {:?}",
                c,
                result
            );
        }
    }

    #[test]
    fn determine_hangul_identifies_valid_vowels() {
        let vowels = "ㅛㅕㅑㅐㅔㅒㅖㅗㅓㅏㅣㅠㅜㅡ";
        for c in vowels.chars() {
            let result = determine_hangul(c);
            assert!(
                result == Letter::Hangul(HangulLetter::Vowel(c)),
                "Failed on vowel: {}; got result: {:?}",
                c,
                result
            );
        }
    }

    #[test]
    fn determine_hangul_double_initials() {
        let compound_letters = "ㄲㄸㅃㅆㅉ";
        for c in compound_letters.chars() {
            let result = determine_hangul(c);
            assert!(
                result == Letter::Hangul(HangulLetter::CompositeConsonant(c)),
                "Failed on compound letter: {}; got result: {:?}",
                c,
                result
            );
        }
    }

    #[test]
    fn determine_hangul_composite_vowels() {
        let compound_letters = "ㅘㅙㅚㅝㅞㅟㅢ";
        for c in compound_letters.chars() {
            let result = determine_hangul(c);
            assert!(
                result == Letter::Hangul(HangulLetter::CompositeVowel(c)),
                "Failed on compound letter: {}; got result: {:?}",
                c,
                result
            );
        }
    }

    #[test]
    fn determine_hangul_composite_finals() {
        let compound_letters = "ㄲㄳㄵㄶㄺㄻㄼㄽㄾㄿㅀㅄ";
        for c in compound_letters.chars() {
            let result = determine_hangul(c);
            assert!(
                result == Letter::Hangul(HangulLetter::CompositeConsonant(c)),
                "Failed on compound letter: {}; got result: {:?}",
                c,
                result
            );
        }
    }

    #[test]
    fn determine_hangul_non_hangul() {
        let non_hangul_chars = "ABCxyz123!@# ";
        for c in non_hangul_chars.chars() {
            let result = determine_hangul(c);
            assert!(
                result == Letter::NonHangul(c),
                "Failed on non-Hangul char: {}; got result: {:?}",
                c,
                result
            );
        }
    }

    #[test]
    fn test_hangul_block_to_char() {
        let block = HangulBlock {
            initial: 'ㄱ',
            vowel: 'ㅏ',
            final_optional: Some('ㄴ'),
        };
        let result = block.to_char();
        assert_eq!(result, Ok('간'));

        let block_no_final = HangulBlock {
            initial: 'ㅂ',
            vowel: 'ㅗ',
            final_optional: None,
        };
        let result_no_final = block_no_final.to_char();
        assert_eq!(result_no_final, Ok('보'));
    }

    #[test]
    fn test_hangul_blocks_vec_to_string() {
        let blocks = vec![
            HangulBlock {
                initial: 'ㅇ',
                vowel: 'ㅏ',
                final_optional: Some('ㄴ'),
            },
            HangulBlock {
                initial: 'ㄴ',
                vowel: 'ㅕ',
                final_optional: Some('ㅇ'),
            },
            HangulBlock {
                initial: 'ㅎ',
                vowel: 'ㅏ',
                final_optional: None,
            },
            HangulBlock {
                initial: 'ㅅ',
                vowel: 'ㅔ',
                final_optional: None,
            },
            HangulBlock {
                initial: 'ㅇ',
                vowel: 'ㅛ',
                final_optional: None,
            },
        ];
        let result = hangul_blocks_vec_to_string(&blocks);
        assert_eq!(result, Ok("안녕하세요".to_string()));
    }
}
