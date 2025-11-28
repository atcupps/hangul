// Jamo sets
const CONSONANTS: &str = "ㅂㅈㄷㄱㅅㅁㄴㅇㄹㅎㅋㅌㅊㅍ";
const COMPOSITE_CONSONANTS: &str = "ㄲㄸㅃㅆㅉㄵㄺㅄㄳㄶㄻㄼㄽㄾㄿㅀ";
const INITIAL_COMPOSITE_CONSONANTS: &str = "ㄲㄸㅃㅆㅉ";
const FINAL_COMPOSITE_CONSONANTS: &str = "ㄲㄵㄺㅄㅆㄳㄶㄻㄼㄽㄾㄿㅀ";
const VOWELS: &str = "ㅛㅕㅑㅐㅔㅒㅖㅗㅓㅏㅣㅠㅜㅡ";
const COMPOSITE_VOWELS: &str = "ㅘㅙㅚㅝㅞㅟㅢ";

// Jamo arithmetic
pub(crate) const S_BASE: u32 = 0xAC00;
pub(crate) const L_BASE: u32 = 0x1100;
pub(crate) const V_BASE: u32 = 0x1161;
pub(crate) const T_BASE: u32 = 0x11A7;
pub(crate) const V_COUNT: u32 = 21;
pub(crate) const T_COUNT: u32 = 28;
pub(crate) const N_COUNT: u32 = V_COUNT * T_COUNT;

pub(crate) fn create_composite_initial(c1: char, c2: char) -> Option<char> {
    match (c1, c2) {
        ('ㄱ', 'ㄱ') => Some('ㄲ'),
        ('ㄷ', 'ㄷ') => Some('ㄸ'),
        ('ㅂ', 'ㅂ') => Some('ㅃ'),
        ('ㅅ', 'ㅅ') => Some('ㅆ'),
        ('ㅈ', 'ㅈ') => Some('ㅉ'),
        _ => None,
    }
}

pub(crate) fn decompose_composite_initial(c: char) -> Option<(char, char)> {
    match c {
        'ㄲ' => Some(('ㄱ', 'ㄱ')),
        'ㄸ' => Some(('ㄷ', 'ㄷ')),
        'ㅃ' => Some(('ㅂ', 'ㅂ')),
        'ㅆ' => Some(('ㅅ', 'ㅅ')),
        'ㅉ' => Some(('ㅈ', 'ㅈ')),
        _ => None,
    }
}

pub(crate) fn create_composite_vowel(v1: char, v2: char) -> Option<char> {
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

pub(crate) fn create_composite_final(c1: char, c2: char) -> Option<char> {
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

pub(crate) fn decompose_composite_final(c: char) -> Option<(char, char)> {
    match c {
        'ㄲ' => Some(('ㄱ', 'ㄱ')),
        'ㄵ' => Some(('ㄴ', 'ㅈ')),
        'ㄺ' => Some(('ㄹ', 'ㄱ')),
        'ㅄ' => Some(('ㅂ', 'ㅅ')),
        'ㅆ' => Some(('ㅅ', 'ㅅ')),
        'ㄳ' => Some(('ㄱ', 'ㅅ')),
        'ㄶ' => Some(('ㄴ', 'ㅎ')),
        'ㄻ' => Some(('ㄹ', 'ㅁ')),
        'ㄼ' => Some(('ㄹ', 'ㅂ')),
        'ㄽ' => Some(('ㄹ', 'ㅅ')),
        'ㄾ' => Some(('ㄹ', 'ㅌ')),
        'ㄿ' => Some(('ㄹ', 'ㅍ')),
        'ㅀ' => Some(('ㄹ', 'ㅎ')),
        _ => None,
    }
}

/// Determines the type of Hangul letter for a given character.
/// Does not work for archaic or non-standard jamo like ᅀ.
/// Classifies a character as Hangul jamo or non-Hangul.
pub fn determine_hangul(c: char) -> Character {
    return if CONSONANTS.contains(c) {
        Character::Hangul(Jamo::Consonant(c))
    } else if VOWELS.contains(c) {
        Character::Hangul(Jamo::Vowel(c))
    } else if COMPOSITE_CONSONANTS.contains(c) {
        Character::Hangul(Jamo::CompositeConsonant(c))
    } else if COMPOSITE_VOWELS.contains(c) {
        Character::Hangul(Jamo::CompositeVowel(c))
    } else {
        Character::NonHangul(c)
    };
}

pub(crate) fn is_valid_double_initial(c: char) -> bool {
    INITIAL_COMPOSITE_CONSONANTS.contains(c)
}

pub(crate) fn is_valid_composite_final(c: char) -> bool {
    FINAL_COMPOSITE_CONSONANTS.contains(c)
}

pub fn modernize_jamo_initial(c: char) -> char {
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

pub fn modernize_jamo_vowel(c: char) -> char {
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

pub fn modernize_jamo_final(c: char) -> char {
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Character {
    NonHangul(char),
    Hangul(Jamo),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Jamo {
    Consonant(char),
    CompositeConsonant(char),
    Vowel(char),
    CompositeVowel(char),
}

impl Jamo {
    pub(crate) fn get_char(&self) -> char {
        match self {
            Jamo::Consonant(c)
            | Jamo::CompositeConsonant(c)
            | Jamo::Vowel(c)
            | Jamo::CompositeVowel(c) => *c,
        }
    }
}
