// Jamo sets

/// String containing all modern Hangul consonants.
pub const CONSONANTS: &str = "ㅂㅈㄷㄱㅅㅁㄴㅇㄹㅎㅋㅌㅊㅍ";

/// String containing all modern Hangul composite consonants.
pub const COMPOSITE_CONSONANTS: &str = "ㄲㄸㅃㅆㅉㄵㄺㅄㄳㄶㄻㄼㄽㄾㄿㅀ";

/// String containing all modern Hangul composite consonants that can be used
/// as initial consonants.
pub const INITIAL_COMPOSITE_CONSONANTS: &str = "ㄲㄸㅃㅆㅉ";

/// String containing all modern Hangul composite consonants that can be used
/// as final consonants.
pub const FINAL_COMPOSITE_CONSONANTS: &str = "ㄲㄵㄺㅄㅆㄳㄶㄻㄼㄽㄾㄿㅀ";

/// String containing all modern Hangul vowels.
pub const VOWELS: &str = "ㅛㅕㅑㅐㅔㅒㅖㅗㅓㅏㅣㅠㅜㅡ";

/// String containing all modern Hangul composite vowels.
pub const COMPOSITE_VOWELS: &str = "ㅘㅙㅚㅝㅞㅟㅢ";

// Jamo arithmetic
pub(crate) const S_BASE: u32 = 0xAC00;
pub(crate) const L_BASE: u32 = 0x1100;
pub(crate) const V_BASE: u32 = 0x1161;
pub(crate) const T_BASE: u32 = 0x11A7;
pub(crate) const V_COUNT: u32 = 21;
pub(crate) const T_COUNT: u32 = 28;
pub(crate) const N_COUNT: u32 = V_COUNT * T_COUNT;

/// Creates a composite initial consonant from two given initial consonants.
/// Returns `None` if the combination is invalid.
/// 
/// **Example:**
/// ```rust
/// let composite = hangul::jamo::create_composite_initial('ㄱ', 'ㄱ');
/// assert_eq!(composite, Some('ㄲ'));
/// ```
pub fn create_composite_initial(c1: char, c2: char) -> Option<char> {
    match (c1, c2) {
        ('ㄱ', 'ㄱ') => Some('ㄲ'),
        ('ㄷ', 'ㄷ') => Some('ㄸ'),
        ('ㅂ', 'ㅂ') => Some('ㅃ'),
        ('ㅅ', 'ㅅ') => Some('ㅆ'),
        ('ㅈ', 'ㅈ') => Some('ㅉ'),
        _ => None,
    }
}

/// Decomposes a composite initial consonant into its constituent consonants.
/// Returns `None` if the character is not a composite initial consonant.
/// 
/// **Example:**
/// ```rust
/// let decomposition = hangul::jamo::decompose_composite_initial('ㄲ');
/// assert_eq!(decomposition, Some(('ㄱ', 'ㄱ')));
/// ```
pub fn decompose_composite_initial(c: char) -> Option<(char, char)> {
    match c {
        'ㄲ' => Some(('ㄱ', 'ㄱ')),
        'ㄸ' => Some(('ㄷ', 'ㄷ')),
        'ㅃ' => Some(('ㅂ', 'ㅂ')),
        'ㅆ' => Some(('ㅅ', 'ㅅ')),
        'ㅉ' => Some(('ㅈ', 'ㅈ')),
        _ => None,
    }
}

/// Creates a composite vowel from two given vowels.
/// Returns `None` if the combination is invalid.
/// 
/// **Example:**
/// ```rust
/// let composite = hangul::jamo::create_composite_vowel('ㅗ', 'ㅏ');
/// assert_eq!(composite, Some('ㅘ'));
/// ```
pub fn create_composite_vowel(v1: char, v2: char) -> Option<char> {
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

/// Decomposes a composite vowel into its constituent vowels.
/// Returns `None` if the character is not a composite vowel.
/// 
/// **Example:**
/// ```rust
/// let decomposition = hangul::jamo::decompose_composite_vowel('ㅘ');
/// assert_eq!(decomposition, Some(('ㅗ', 'ㅏ')));
/// ```
pub fn decompose_composite_vowel(c: char) -> Option<(char, char)> {
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

/// Creates a composite final consonant from two given final consonants.
/// Returns `None` if the combination is invalid.
/// 
/// **Example:**
/// ```rust
/// let composite = hangul::jamo::create_composite_final('ㄱ', 'ㅅ');
/// assert_eq!(composite, Some('ㄳ'));
/// ```
pub fn create_composite_final(c1: char, c2: char) -> Option<char> {
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

/// Decomposes a composite final consonant into its constituent consonants.
/// Returns `None` if the character is not a composite final consonant.
/// 
/// **Example:**
/// ```rust
/// let decomposition = hangul::jamo::decompose_composite_final('ㄳ');
/// assert_eq!(decomposition, Some(('ㄱ', 'ㅅ')));
/// ```
pub fn decompose_composite_final(c: char) -> Option<(char, char)> {
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
/// 
/// **Example:**
/// ```rust
/// use hangul::jamo::{determine_hangul, Character, Jamo};
/// 
/// // Valid Hangul consonant
/// assert_eq!(
///     determine_hangul('ㄱ'),
///     Character::Hangul(Jamo::Consonant('ㄱ'))
/// );
/// 
/// // Valid Hangul vowel
/// assert_eq!(
///     determine_hangul('ㅏ'),
///     Character::Hangul(Jamo::Vowel('ㅏ'))
/// );
/// 
/// // Valid composite consonant
/// assert_eq!(
///     determine_hangul('ㄲ'),
///     Character::Hangul(Jamo::CompositeConsonant('ㄲ'))
/// );
/// 
/// // Valid composite vowel
/// assert_eq!(
///     determine_hangul('ㅘ'),
///     Character::Hangul(Jamo::CompositeVowel('ㅘ'))
/// );
/// 
/// // Non-Hangul character
/// assert_eq!(
///     determine_hangul('A'),
///     Character::NonHangul('A')
/// );
/// ```
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

/// Converts compatibility jamo to modern jamo, specifically for 
/// initial consonants or initial composite consonants.
/// 
/// What are compatibility and modern jamo? In Unicode, Hangul jamo characteres
/// are represented in two different blocks: the "Hangul Jamo" block (U+1100 to U+11FF)
/// and the "Hangul Compatibility Jamo" block (U+3130 to U+318F).
/// The former contains the modern jamo used for composing syllables, while the latter
/// includes characters for compatibility with older standards and encodings.
/// 
/// As an example, both U+1100 and U+3131 represent the Hangul consonant "Giyeok" (ㄱ),
/// but U+1100 is the modern jamo used in syllable composition, while
/// U+3131 is the compatibility jamo.
/// 
/// In order to properly compose Hangul syllables as Unicode characters, it is
/// necessary to convert any compatibility jamo into their modern equivalents.
/// The math done to compose syllables in this crate relies on the use of
/// modern Jamo. The APIs for converting `HangulBlock`s or composing blocks
/// or words or strings in this crate automatically handle this conversion,
/// but the function is provided for users who want to manually work with Jamo
/// characters.
/// 
/// This function maps compatibility jamo characters to their modern equivalents.
/// If the input character is not a compatibility jamo, it is returned unchanged
/// (including if it is not a Hangul jamo at all).
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

/// Converts compatibility jamo to modern jamo, specifically for
/// vowels or composite vowels.
/// 
/// What are compatibility and modern jamo? In Unicode, Hangul jamo characteres
/// are represented in two different blocks: the "Hangul Jamo" block (U+1100 to U+11FF)
/// and the "Hangul Compatibility Jamo" block (U+3130 to U+318F).
/// The former contains the modern jamo used for composing syllables, while the latter
/// includes characters for compatibility with older standards and encodings.
/// 
/// As an example, both U+1100 and U+3131 represent the Hangul consonant "Giyeok" (ㄱ),
/// but U+1100 is the modern jamo used in syllable composition, while
/// U+3131 is the compatibility jamo.
/// 
/// In order to properly compose Hangul syllables as Unicode characters, it is
/// necessary to convert any compatibility jamo into their modern equivalents.
/// The math done to compose syllables in this crate relies on the use of
/// modern Jamo. The APIs for converting `HangulBlock`s or composing blocks
/// or words or strings in this crate automatically handle this conversion,
/// but the function is provided for users who want to manually work with Jamo
/// characters.
/// 
/// This function maps compatibility jamo characters to their modern equivalents.
/// If the input character is not a compatibility jamo, it is returned unchanged
/// (including if it is not a Hangul jamo at all).
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

/// Converts compatibility jamo to modern jamo, specifically for
/// final consonants or final composite consonants.
/// 
/// What are compatibility and modern jamo? In Unicode, Hangul jamo characteres
/// are represented in two different blocks: the "Hangul Jamo" block (U+1100 to U+11FF)
/// and the "Hangul Compatibility Jamo" block (U+3130 to U+318F).
/// The former contains the modern jamo used for composing syllables, while the latter
/// includes characters for compatibility with older standards and encodings.
/// 
/// As an example, both U+1100 and U+3131 represent the Hangul consonant "Giyeok" (ㄱ),
/// but U+1100 is the modern jamo used in syllable composition, while
/// U+3131 is the compatibility jamo.
/// 
/// In order to properly compose Hangul syllables as Unicode characters, it is
/// necessary to convert any compatibility jamo into their modern equivalents.
/// The math done to compose syllables in this crate relies on the use of
/// modern Jamo. The APIs for converting `HangulBlock`s or composing blocks
/// or words or strings in this crate automatically handle this conversion,
/// but the function is provided for users who want to manually work with Jamo
/// characters.
/// 
/// This function maps compatibility jamo characters to their modern equivalents.
/// If the input character is not a compatibility jamo, it is returned unchanged
/// (including if it is not a Hangul jamo at all).
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

/// An enum representing either a Hangul Jamo character or a non-Hangul
/// character.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Character {
    NonHangul(char),
    Hangul(Jamo),
}

/// An enum representing the different types of Hangul Jamo characters:
/// consonants, composite consonants, vowels, and composite vowels.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Jamo {
    Consonant(char),
    CompositeConsonant(char),
    Vowel(char),
    CompositeVowel(char),
}

impl Jamo {
    /// Returns the underlying character of the Jamo.
    /// 
    /// **Example:**
    /// ```rust
    /// use hangul::jamo::Jamo;
    /// let jamo = Jamo::Consonant('ㄱ');
    /// assert_eq!(jamo.get_char(), 'ㄱ');
    /// ```
    pub fn get_char(&self) -> char {
        match self {
            Jamo::Consonant(c)
            | Jamo::CompositeConsonant(c)
            | Jamo::Vowel(c)
            | Jamo::CompositeVowel(c) => *c,
        }
    }
}
