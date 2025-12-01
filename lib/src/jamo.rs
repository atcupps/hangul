pub enum JamoUnicodeEra {
    Modern,
    Compatibility,
    NonStandardModern,
    NonStandardCompatibility,
    NonHangul,
}

impl JamoUnicodeEra {
    pub fn from_char(c: char) -> JamoUnicodeEra {
        match c as u32 {
            0x1100..=0x1112 | 0x1161..=0x1175 | 0x11A8..=0x11C2 => JamoUnicodeEra::Modern,
            0x3130..=0x3163 => JamoUnicodeEra::Compatibility,
            0x1113..=0x1160 | 0x1176..=0x11A7 | 0x11C3..=0x11FF => JamoUnicodeEra::NonStandardModern,
            0x3164..=0x318F => JamoUnicodeEra::NonStandardCompatibility,
            _ => JamoUnicodeEra::NonHangul,
        }
    }
}

// Jamo arithmetic
pub(crate) const S_BASE: u32 = 0xAC00;
pub(crate) const L_BASE: u32 = 0x1100;
pub(crate) const V_BASE: u32 = 0x1161;
pub(crate) const T_BASE: u32 = 0x11A7;
pub(crate) const V_COUNT: u32 = 21;
pub(crate) const T_COUNT: u32 = 28;
pub(crate) const N_COUNT: u32 = V_COUNT * T_COUNT;

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
pub fn modernized_jamo_initial(c: char) -> char {
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
pub fn modernized_jamo_vowel(c: char) -> char {
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
pub fn modernized_jamo_final(c: char) -> char {
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

/// Converts a modern jamo character to its compatibility jamo equivalent.
/// If the input character is not a modern jamo, it is returned unchanged
/// (including if it is not a Hangul jamo at all).
/// 
/// For more info on modern and compatibility jamo, see the documentation
/// for `modernized_jamo_initial`, `modernized_jamo_vowel`, 
/// or `modernized_jamo_final`.
pub fn modern_to_compatibility_jamo(c: char) -> char {
    match c {
        // Initial consonants
        '\u{1100}' => '\u{3131}', // ㄱ
        '\u{1101}' => '\u{3132}', // ㄲ
        '\u{1102}' => '\u{3134}', // ㄴ
        '\u{1103}' => '\u{3137}', // ㄷ
        '\u{1104}' => '\u{3138}', // ㄸ
        '\u{1105}' => '\u{3139}', // ㄹ
        '\u{1106}' => '\u{3141}', // ㅁ
        '\u{1107}' => '\u{3142}', // ㅂ
        '\u{1108}' => '\u{3143}', // ㅃ
        '\u{1109}' => '\u{3145}', // ㅅ
        '\u{110A}' => '\u{3146}', // ㅆ
        '\u{110B}' => '\u{3147}', // ㅇ
        '\u{110C}' => '\u{3148}', // ㅈ
        '\u{110D}' => '\u{3149}', // ㅉ
        '\u{110E}' => '\u{314A}', // ㅊ
        '\u{110F}' => '\u{314B}', // ㅋ
        '\u{1110}' => '\u{314C}', // ㅌ
        '\u{1111}' => '\u{314D}', // ㅍ
        '\u{1112}' => '\u{314E}', // ㅎ

        // Vowels
        '\u{1161}' => '\u{314F}', // ㅏ
        '\u{1162}' => '\u{3150}', // ㅐ
        '\u{1163}' => '\u{3151}', // ㅑ
        '\u{1164}' => '\u{3152}', // ㅒ
        '\u{1165}' => '\u{3153}', // ㅓ
        '\u{1166}' => '\u{3154}', // ㅔ
        '\u{1167}' => '\u{3155}', // ㅕ
        '\u{1168}' => '\u{3156}', // ㅖ
        '\u{1169}' => '\u{3157}', // ㅗ
        '\u{116A}' => '\u{3158}', // ㅘ
        '\u{116B}' => '\u{3159}', // ㅙ
        '\u{116C}' => '\u{315A}', // ㅚ
        '\u{116D}' => '\u{315B}', // ㅛ
        '\u{116E}' => '\u{315C}', // ㅜ
        '\u{116F}' => '\u{315D}', // ㅝ
        '\u{1170}' => '\u{315E}', // ㅞ
        '\u{1171}' => '\u{315F}', // ㅟ
        '\u{1172}' => '\u{3160}', // ㅠ
        '\u{1173}' => '\u{3161}', // ㅡ
        '\u{1174}' => '\u{3162}', // ㅢ
        '\u{1175}' => '\u{3163}', // ㅣ

        // Final consonants
        '\u{11A8}' => '\u{3131}', // ㄱ
        '\u{11A9}' => '\u{3132}', // ㄲ
        '\u{11AA}' => '\u{3133}', // ㄳ
        '\u{11AB}' => '\u{3134}', // ㄴ
        '\u{11AC}' => '\u{3135}', // ㄵ
        '\u{11AD}' => '\u{3136}', // ㄶ
        '\u{11AE}' => '\u{3137}', // ㄷ
        '\u{11AF}' => '\u{3139}', // ㄹ
        '\u{11B0}' => '\u{313A}', // ㄺ
        '\u{11B1}' => '\u{313B}', // ㄻ
        '\u{11B2}' => '\u{313C}', // ㄼ
        '\u{11B3}' => '\u{313D}', // ㄽ
        '\u{11B4}' => '\u{313E}', // ㄾ
        '\u{11B5}' => '\u{313F}', // ㄿ
        '\u{11B6}' => '\u{3140}', // ㅀ
        '\u{11B7}' => '\u{3141}', // ㅁ
        '\u{11B8}' => '\u{3142}', // ㅂ
        '\u{11B9}' => '\u{3144}', // ㅄ
        '\u{11BA}' => '\u{3145}', // ㅅ
        '\u{11BB}' => '\u{3146}', // ㅆ
        '\u{11BC}' => '\u{3147}', // ㅇ
        '\u{11BD}' => '\u{3148}', // ㅈ
        '\u{11BE}' => '\u{314A}', // ㅊ
        '\u{11BF}' => '\u{314B}', // ㅋ
        '\u{11C0}' => '\u{314C}', // ㅌ
        '\u{11C1}' => '\u{314D}', // ㅍ
        '\u{11C2}' => '\u{314E}', // ㅎ

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

impl Character {
    /// Determines the type of Hangul letter for a given character.
    /// Does not work for archaic or non-standard jamo like ᅀ.
    /// Classifies a character as Hangul jamo or non-Hangul and
    /// returns the appropriate `Character` enum variant.
    ///
    /// **Example:**
    /// ```rust
    /// use hangul::jamo::{Character, Jamo, JamoConsonantSingular, JamoVowelSingular, JamoVowelComposite, JamoConsonantComposite};
    ///
    /// // Valid Hangul consonant
    /// assert_eq!(
    ///     Character::from_char('ㄱ').unwrap(),
    ///     Character::Hangul(Jamo::Consonant(JamoConsonantSingular::Giyeok))
    /// );
    ///
    /// // Valid Hangul vowel
    /// assert_eq!(
    ///     Character::from_char('ㅏ').unwrap(),
    ///     Character::Hangul(Jamo::Vowel(JamoVowelSingular::A))
    /// );
    ///
    /// // Valid composite consonant
    /// assert_eq!(
    ///     Character::from_char('ㄲ').unwrap(),
    ///     Character::Hangul(Jamo::CompositeConsonant(JamoConsonantComposite::GiyeokGiyeok))
    /// );
    ///
    /// // Valid composite vowel
    /// assert_eq!(
    ///     Character::from_char('ㅘ').unwrap(),
    ///     Character::Hangul(Jamo::CompositeVowel(JamoVowelComposite::Wa))
    /// );
    ///
    /// // Non-Hangul character
    /// assert_eq!(
    ///     Character::from_char('A').unwrap(),
    ///     Character::NonHangul('A')
    /// );
    /// ```
    pub fn from_char(c: char) -> Result<Self, String> {
        match JamoUnicodeEra::from_char(c) {
            JamoUnicodeEra::Modern => {
                let cc = modern_to_compatibility_jamo(c);
                Self::from_compatibility_jamo(cc)
            },
            JamoUnicodeEra::Compatibility => Self::from_compatibility_jamo(c),
            _ => Ok(Character::NonHangul(c)),
        }
    }

    fn from_compatibility_jamo(c: char) -> Result<Self, String> {
        Ok(Self::Hangul(Jamo::from_compatibility_jamo(c)?))
    }

    /// Returns the Jamo if the character is a Hangul jamo, or `None` otherwise.
    pub fn jamo(&self) -> Option<&Jamo> {
        match self {
            Character::Hangul(jamo) => Some(jamo),
            Character::NonHangul(_) => None,
        }
    }
}

/// An enum representing the different types of Hangul Jamo characters:
/// consonants, composite consonants, vowels, and composite vowels.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Jamo {
    Consonant(JamoConsonantSingular),
    CompositeConsonant(JamoConsonantComposite),
    Vowel(JamoVowelSingular),
    CompositeVowel(JamoVowelComposite),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JamoConsonantSingular {
    /// ㄱ
    Giyeok,
    /// ㄴ
    Nieun,
    /// ㄷ
    Digeut,
    /// ㄹ
    Rieul,
    /// ㅁ
    Mieum,
    /// ㅂ
    Bieup,
    /// ㅅ
    Siot,
    /// ㅇ
    Ieung,
    /// ㅈ
    Jieut,
    /// ㅊ
    Chieut,
    /// ㅋ
    Kieuk,
    /// ㅌ
    Tieut,
    /// ㅍ
    Pieup,
    /// ㅎ
    Hieut,
}

impl JamoConsonantSingular {
    pub fn char_modern_initial(&self) -> char {
        match self {
            JamoConsonantSingular::Giyeok => '\u{1100}',
            JamoConsonantSingular::Nieun => '\u{1102}',
            JamoConsonantSingular::Digeut => '\u{1103}',
            JamoConsonantSingular::Rieul => '\u{1105}',
            JamoConsonantSingular::Mieum => '\u{1106}',
            JamoConsonantSingular::Bieup => '\u{1107}',
            JamoConsonantSingular::Siot => '\u{1109}',
            JamoConsonantSingular::Ieung => '\u{110B}',
            JamoConsonantSingular::Jieut => '\u{110C}',
            JamoConsonantSingular::Chieut => '\u{110E}',
            JamoConsonantSingular::Kieuk => '\u{110F}',
            JamoConsonantSingular::Tieut => '\u{1110}',
            JamoConsonantSingular::Pieup => '\u{1111}',
            JamoConsonantSingular::Hieut => '\u{1112}',
        }
    }

    pub fn char_modern_final(&self) -> char {
        match self {
            JamoConsonantSingular::Giyeok => '\u{11A8}',
            JamoConsonantSingular::Nieun => '\u{11AB}',
            JamoConsonantSingular::Digeut => '\u{11AE}',
            JamoConsonantSingular::Rieul => '\u{11AF}',
            JamoConsonantSingular::Mieum => '\u{11B7}',
            JamoConsonantSingular::Bieup => '\u{11B8}',
            JamoConsonantSingular::Siot => '\u{11BA}',
            JamoConsonantSingular::Ieung => '\u{11BC}',
            JamoConsonantSingular::Jieut => '\u{11BD}',
            JamoConsonantSingular::Chieut => '\u{11BE}',
            JamoConsonantSingular::Kieuk => '\u{11BF}',
            JamoConsonantSingular::Tieut => '\u{11C0}',
            JamoConsonantSingular::Pieup => '\u{11C1}',
            JamoConsonantSingular::Hieut => '\u{11C2}',
        }
    }

    pub fn char_compatibility(&self) -> char {
        match self {
            JamoConsonantSingular::Giyeok => 'ㄱ',
            JamoConsonantSingular::Nieun => 'ㄴ',
            JamoConsonantSingular::Digeut => 'ㄷ',
            JamoConsonantSingular::Rieul => 'ㄹ',
            JamoConsonantSingular::Mieum => 'ㅁ',
            JamoConsonantSingular::Bieup => 'ㅂ',
            JamoConsonantSingular::Siot => 'ㅅ',
            JamoConsonantSingular::Ieung => 'ㅇ',
            JamoConsonantSingular::Jieut => 'ㅈ',
            JamoConsonantSingular::Chieut => 'ㅊ',
            JamoConsonantSingular::Kieuk => 'ㅋ',
            JamoConsonantSingular::Tieut => 'ㅌ',
            JamoConsonantSingular::Pieup => 'ㅍ',
            JamoConsonantSingular::Hieut => 'ㅎ',
        }
    }

    pub fn combine_for_initial(&self, other: &JamoConsonantSingular) -> Option<JamoConsonantComposite> {
        match (self, other) {
            (JamoConsonantSingular::Giyeok, JamoConsonantSingular::Giyeok) => Some(JamoConsonantComposite::GiyeokGiyeok),
            (JamoConsonantSingular::Digeut, JamoConsonantSingular::Digeut) => Some(JamoConsonantComposite::DigeutDigeut),
            (JamoConsonantSingular::Bieup, JamoConsonantSingular::Bieup) => Some(JamoConsonantComposite::BieupBieup),
            (JamoConsonantSingular::Siot, JamoConsonantSingular::Siot) => Some(JamoConsonantComposite::SiotSiot),
            (JamoConsonantSingular::Jieut, JamoConsonantSingular::Jieut) => Some(JamoConsonantComposite::JieutJieut),
            _ => None,
        }
    }

    pub fn combine_for_final(&self, other: &JamoConsonantSingular) -> Option<JamoConsonantComposite> {
        match (self, other) {
            (JamoConsonantSingular::Giyeok, JamoConsonantSingular::Siot) => Some(JamoConsonantComposite::GiyeokSiot),
            (JamoConsonantSingular::Nieun, JamoConsonantSingular::Jieut) => Some(JamoConsonantComposite::NieunJieut),
            (JamoConsonantSingular::Nieun, JamoConsonantSingular::Hieut) => Some(JamoConsonantComposite::NieunHieut),
            (JamoConsonantSingular::Rieul, JamoConsonantSingular::Giyeok) => Some(JamoConsonantComposite::RieulGiyeok),
            (JamoConsonantSingular::Rieul, JamoConsonantSingular::Mieum) => Some(JamoConsonantComposite::RieulMieum),
            (JamoConsonantSingular::Rieul, JamoConsonantSingular::Bieup) => Some(JamoConsonantComposite::RieulBieup),
            (JamoConsonantSingular::Rieul, JamoConsonantSingular::Siot) => Some(JamoConsonantComposite::RieulSiot),
            (JamoConsonantSingular::Rieul, JamoConsonantSingular::Tieut) => Some(JamoConsonantComposite::RieulTieut),
            (JamoConsonantSingular::Rieul, JamoConsonantSingular::Pieup) => Some(JamoConsonantComposite::RieulPieup),
            (JamoConsonantSingular::Rieul, JamoConsonantSingular::Hieut) => Some(JamoConsonantComposite::RieulHieut),
            (JamoConsonantSingular::Giyeok, JamoConsonantSingular::Giyeok) => Some(JamoConsonantComposite::GiyeokGiyeok),
            (JamoConsonantSingular::Siot, JamoConsonantSingular::Siot) => Some(JamoConsonantComposite::SiotSiot),
            (JamoConsonantSingular::Bieup, JamoConsonantSingular::Siot) => Some(JamoConsonantComposite::BieupSiot),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JamoConsonantComposite {
    /// ㄳ
    GiyeokSiot,
    /// ㄵ
    NieunJieut,
    /// ㄶ
    NieunHieut,
    /// ㄺ
    RieulGiyeok,
    /// ㄻ
    RieulMieum,
    /// ㄼ
    RieulBieup,
    /// ㄽ
    RieulSiot,
    /// ㄾ
    RieulTieut,
    /// ㄿ
    RieulPieup,
    /// ㅀ
    RieulHieut,
    /// ㄲ
    GiyeokGiyeok,
    /// ㄸ
    DigeutDigeut,
    /// ㅃ
    BieupBieup,
    /// ㅆ
    SiotSiot,
    /// ㅉ
    JieutJieut,
    /// ㅄ
    BieupSiot,
}

impl JamoConsonantComposite {
    pub fn char_modern_initial(&self) -> Option<char> {
        match self {
            JamoConsonantComposite::GiyeokGiyeok => Some('\u{1101}'),
            JamoConsonantComposite::DigeutDigeut => Some('\u{1104}'),
            JamoConsonantComposite::BieupBieup => Some('\u{1108}'),
            JamoConsonantComposite::SiotSiot => Some('\u{110A}'),
            JamoConsonantComposite::JieutJieut => Some('\u{110D}'),
            _ => None,
        }
    }

    pub fn char_modern_final(&self) -> Option<char> {
        match self {
            JamoConsonantComposite::GiyeokSiot => Some('\u{11AA}'),
            JamoConsonantComposite::NieunJieut => Some('\u{11AC}'),
            JamoConsonantComposite::NieunHieut => Some('\u{11AD}'),
            JamoConsonantComposite::RieulGiyeok => Some('\u{11B0}'),
            JamoConsonantComposite::RieulMieum => Some('\u{11B1}'),
            JamoConsonantComposite::RieulBieup => Some('\u{11B2}'),
            JamoConsonantComposite::RieulSiot => Some('\u{11B3}'),
            JamoConsonantComposite::RieulTieut => Some('\u{11B4}'),
            JamoConsonantComposite::RieulPieup => Some('\u{11B5}'),
            JamoConsonantComposite::RieulHieut => Some('\u{11B6}'),
            JamoConsonantComposite::GiyeokGiyeok => Some('\u{11A9}'),
            JamoConsonantComposite::SiotSiot => Some('\u{11BB}'),
            JamoConsonantComposite::BieupSiot => Some('\u{11B9}'),
            _ => None,
        }
    }

    pub fn char_compatibility(&self) -> char {
        match self {
            JamoConsonantComposite::GiyeokSiot => 'ㄳ',
            JamoConsonantComposite::NieunJieut => 'ㄵ',
            JamoConsonantComposite::NieunHieut => 'ㄶ',
            JamoConsonantComposite::RieulGiyeok => 'ㄺ',
            JamoConsonantComposite::RieulMieum => 'ㄻ',
            JamoConsonantComposite::RieulBieup => 'ㄼ',
            JamoConsonantComposite::RieulSiot => 'ㄽ',
            JamoConsonantComposite::RieulTieut => 'ㄾ',
            JamoConsonantComposite::RieulPieup => 'ㄿ',
            JamoConsonantComposite::RieulHieut => 'ㅀ',
            JamoConsonantComposite::GiyeokGiyeok => 'ㄲ',
            JamoConsonantComposite::DigeutDigeut => 'ㄸ',
            JamoConsonantComposite::BieupBieup => 'ㅃ',
            JamoConsonantComposite::SiotSiot => 'ㅆ',
            JamoConsonantComposite::JieutJieut => 'ㅉ',
            JamoConsonantComposite::BieupSiot => 'ㅄ',
        }
    }

    pub fn decompose(&self) -> (Jamo, Jamo) {
        match self {
            JamoConsonantComposite::GiyeokSiot => (
                Jamo::Consonant(JamoConsonantSingular::Giyeok),
                Jamo::Consonant(JamoConsonantSingular::Siot),
            ),
            JamoConsonantComposite::NieunJieut => (
                Jamo::Consonant(JamoConsonantSingular::Nieun),
                Jamo::Consonant(JamoConsonantSingular::Jieut),
            ),
            JamoConsonantComposite::NieunHieut => (
                Jamo::Consonant(JamoConsonantSingular::Nieun),
                Jamo::Consonant(JamoConsonantSingular::Hieut),
            ),
            JamoConsonantComposite::RieulGiyeok => (
                Jamo::Consonant(JamoConsonantSingular::Rieul),
                Jamo::Consonant(JamoConsonantSingular::Giyeok),
            ),
            JamoConsonantComposite::RieulMieum => (
                Jamo::Consonant(JamoConsonantSingular::Rieul),
                Jamo::Consonant(JamoConsonantSingular::Mieum),
            ),
            JamoConsonantComposite::RieulBieup => (
                Jamo::Consonant(JamoConsonantSingular::Rieul),
                Jamo::Consonant(JamoConsonantSingular::Bieup),
            ),
            JamoConsonantComposite::RieulSiot => (
                Jamo::Consonant(JamoConsonantSingular::Rieul),
                Jamo::Consonant(JamoConsonantSingular::Siot),
            ),
            JamoConsonantComposite::RieulTieut => (
                Jamo::Consonant(JamoConsonantSingular::Rieul),
                Jamo::Consonant(JamoConsonantSingular::Tieut),
            ),
            JamoConsonantComposite::RieulPieup => (
                Jamo::Consonant(JamoConsonantSingular::Rieul),
                Jamo::Consonant(JamoConsonantSingular::Pieup),
            ),
            JamoConsonantComposite::RieulHieut => (
                Jamo::Consonant(JamoConsonantSingular::Rieul),
                Jamo::Consonant(JamoConsonantSingular::Hieut),
            ),
            JamoConsonantComposite::GiyeokGiyeok => (
                Jamo::Consonant(JamoConsonantSingular::Giyeok),
                Jamo::Consonant(JamoConsonantSingular::Giyeok),
            ),
            JamoConsonantComposite::DigeutDigeut => (
                Jamo::Consonant(JamoConsonantSingular::Digeut),
                Jamo::Consonant(JamoConsonantSingular::Digeut),
            ),
            JamoConsonantComposite::BieupBieup => (
                Jamo::Consonant(JamoConsonantSingular::Bieup),
                Jamo::Consonant(JamoConsonantSingular::Bieup),
            ),
            JamoConsonantComposite::SiotSiot => (
                Jamo::Consonant(JamoConsonantSingular::Siot),
                Jamo::Consonant(JamoConsonantSingular::Siot),
            ),
            JamoConsonantComposite::JieutJieut => (
                Jamo::Consonant(JamoConsonantSingular::Jieut),
                Jamo::Consonant(JamoConsonantSingular::Jieut),
            ),
            JamoConsonantComposite::BieupSiot => (
                Jamo::Consonant(JamoConsonantSingular::Bieup),
                Jamo::Consonant(JamoConsonantSingular::Siot),
            ),
        }
    }

    pub fn is_valid_initial(&self) -> bool {
        match self {
            JamoConsonantComposite::GiyeokGiyeok
            | JamoConsonantComposite::DigeutDigeut
            | JamoConsonantComposite::BieupBieup
            | JamoConsonantComposite::SiotSiot
            | JamoConsonantComposite::JieutJieut => true,
            _ => false,
        }
    }

    pub fn is_valid_final(&self) -> bool {
        match self {
            JamoConsonantComposite::GiyeokSiot
            | JamoConsonantComposite::NieunJieut
            | JamoConsonantComposite::NieunHieut
            | JamoConsonantComposite::RieulGiyeok
            | JamoConsonantComposite::RieulMieum
            | JamoConsonantComposite::RieulBieup
            | JamoConsonantComposite::RieulSiot
            | JamoConsonantComposite::RieulTieut
            | JamoConsonantComposite::RieulPieup
            | JamoConsonantComposite::RieulHieut
            | JamoConsonantComposite::GiyeokGiyeok
            | JamoConsonantComposite::SiotSiot
            | JamoConsonantComposite::BieupSiot => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JamoVowelSingular {
    /// ㅏ
    A,
    /// ㅐ
    Ae,
    /// ㅑ
    Ya,
    /// ㅒ
    Yae,
    /// ㅓ
    Eo,
    /// ㅔ
    E,
    /// ㅕ
    Yeo,
    /// ㅖ
    Ye,
    /// ㅗ
    O,
    /// ㅛ
    Yo,
    /// ㅜ
    U,
    /// ㅠ
    Yu,
    /// ㅡ
    Eu,
    /// ㅣ
    I,
}

impl JamoVowelSingular {
    pub fn char_modern(&self) -> char {
        match self {
            JamoVowelSingular::A => '\u{1161}',
            JamoVowelSingular::Ae => '\u{1162}',
            JamoVowelSingular::Ya => '\u{1163}',
            JamoVowelSingular::Yae => '\u{1164}',
            JamoVowelSingular::Eo => '\u{1165}',
            JamoVowelSingular::E => '\u{1166}',
            JamoVowelSingular::Yeo => '\u{1167}',
            JamoVowelSingular::Ye => '\u{1168}',
            JamoVowelSingular::O => '\u{1169}',
            JamoVowelSingular::Yo => '\u{116D}',
            JamoVowelSingular::U => '\u{116E}',
            JamoVowelSingular::Yu => '\u{1172}',
            JamoVowelSingular::Eu => '\u{1173}',
            JamoVowelSingular::I => '\u{1175}',
        }
    }

    pub fn char_compatibility(&self) -> char {
        match self {
            JamoVowelSingular::A => 'ㅏ',
            JamoVowelSingular::Ae => 'ㅐ',
            JamoVowelSingular::Ya => 'ㅑ',
            JamoVowelSingular::Yae => 'ㅒ',
            JamoVowelSingular::Eo => 'ㅓ',
            JamoVowelSingular::E => 'ㅔ',
            JamoVowelSingular::Yeo => 'ㅕ',
            JamoVowelSingular::Ye => 'ㅖ',
            JamoVowelSingular::O => 'ㅗ',
            JamoVowelSingular::Yo => 'ㅛ',
            JamoVowelSingular::U => 'ㅜ',
            JamoVowelSingular::Yu => 'ㅠ',
            JamoVowelSingular::Eu => 'ㅡ',
            JamoVowelSingular::I => 'ㅣ',
        }
    }

    pub fn combine(&self, other: &JamoVowelSingular) -> Option<JamoVowelComposite> {
        match (self, other) {
            (JamoVowelSingular::O, JamoVowelSingular::A) => Some(JamoVowelComposite::Wa),
            (JamoVowelSingular::O, JamoVowelSingular::Ae) => Some(JamoVowelComposite::Wae),
            (JamoVowelSingular::O, JamoVowelSingular::I) => Some(JamoVowelComposite::Oe),
            (JamoVowelSingular::U, JamoVowelSingular::Eo) => Some(JamoVowelComposite::Wo),
            (JamoVowelSingular::U, JamoVowelSingular::E) => Some(JamoVowelComposite::We),
            (JamoVowelSingular::U, JamoVowelSingular::I) => Some(JamoVowelComposite::Wi),
            (JamoVowelSingular::Eu, JamoVowelSingular::I) => Some(JamoVowelComposite::Ui),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JamoVowelComposite {
    /// ㅘ
    Wa,
    /// ㅙ
    Wae,
    /// ㅚ
    Oe,
    /// ㅝ
    Wo,
    /// ㅞ
    We,
    /// ㅟ
    Wi,
    /// ㅢ
    Ui,
}

impl JamoVowelComposite {
    pub fn char_modern(&self) -> char {
        match self {
            JamoVowelComposite::Wa => '\u{116A}',
            JamoVowelComposite::Wae => '\u{116B}',
            JamoVowelComposite::Oe => '\u{116C}',
            JamoVowelComposite::Wo => '\u{116F}',
            JamoVowelComposite::We => '\u{1170}',
            JamoVowelComposite::Wi => '\u{1171}',
            JamoVowelComposite::Ui => '\u{1174}',
        }
    }

    pub fn char_compatibility(&self) -> char {
        match self {
            JamoVowelComposite::Wa => 'ㅘ',
            JamoVowelComposite::Wae => 'ㅙ',
            JamoVowelComposite::Oe => 'ㅚ',
            JamoVowelComposite::Wo => 'ㅝ',
            JamoVowelComposite::We => 'ㅞ',
            JamoVowelComposite::Wi => 'ㅟ',
            JamoVowelComposite::Ui => 'ㅢ',
        }
    }

    pub fn decompose(&self) -> (Jamo, Jamo) {
        match self {
            JamoVowelComposite::Wa => (
                Jamo::Vowel(JamoVowelSingular::O),
                Jamo::Vowel(JamoVowelSingular::A),
            ),
            JamoVowelComposite::Wae => (
                Jamo::Vowel(JamoVowelSingular::O),
                Jamo::Vowel(JamoVowelSingular::Ae),
            ),
            JamoVowelComposite::Oe => (
                Jamo::Vowel(JamoVowelSingular::O),
                Jamo::Vowel(JamoVowelSingular::I),
            ),
            JamoVowelComposite::Wo => (
                Jamo::Vowel(JamoVowelSingular::U),
                Jamo::Vowel(JamoVowelSingular::Eo),
            ),
            JamoVowelComposite::We => (
                Jamo::Vowel(JamoVowelSingular::U),
                Jamo::Vowel(JamoVowelSingular::E),
            ),
            JamoVowelComposite::Wi => (
                Jamo::Vowel(JamoVowelSingular::U),
                Jamo::Vowel(JamoVowelSingular::I),
            ),
            JamoVowelComposite::Ui => (
                Jamo::Vowel(JamoVowelSingular::Eu),
                Jamo::Vowel(JamoVowelSingular::I),
            ),
        }
    }
}

pub enum JamoPosition {
    Initial,
    Vowel,
    Final,
}

impl Jamo {
    /// Returns the compatibility jamo character for this Jamo.
    /// This is a different Unicode codepoint than the modernized version.
    ///
    /// **Example:**
    /// ```rust
    /// use hangul::jamo::{Jamo, JamoConsonantSingular};
    /// let jamo = Jamo::Consonant(JamoConsonantSingular::Giyeok);
    /// assert_eq!(jamo.char_compatibility(), 'ㄱ');
    /// ```
    pub fn char_compatibility(&self) -> char {
        match self {
            Jamo::Consonant(c) => c.char_compatibility(),
            Jamo::CompositeConsonant(c) => c.char_compatibility(),
            Jamo::Vowel(c) => c.char_compatibility(),
            Jamo::CompositeVowel(c) => c.char_compatibility(),
        }
    }

    /// Returns the modern jamo character for this Jamo.
    /// This is a different Unicode codepoint than the compatibility version.
    ///
    /// **Example:**
    /// ```rust
    /// use hangul::jamo::{Jamo, JamoConsonantSingular, JamoPosition};
    /// let jamo = Jamo::Consonant(JamoConsonantSingular::Giyeok);
    /// assert_eq!(jamo.char_modern(JamoPosition::Initial), Some('ᄀ'));
    /// ```
    pub fn char_modern(&self, position: JamoPosition) -> Option<char> {
        match self {
            Jamo::Consonant(c) => match position {
                JamoPosition::Initial => Some(c.char_modern_initial()),
                JamoPosition::Final => Some(c.char_modern_final()),
                JamoPosition::Vowel => None,
            },
            Jamo::CompositeConsonant(c) => match position {
                JamoPosition::Initial => c.char_modern_initial(),
                JamoPosition::Final => c.char_modern_final(),
                JamoPosition::Vowel => None,
            },
            Jamo::Vowel(c) => match position {
                JamoPosition::Vowel => Some(c.char_modern()),
                _ => None,
            },
            Jamo::CompositeVowel(c) => match position {
                JamoPosition::Vowel => Some(c.char_modern()),
                _ => None,
            },
        }
    }

    pub fn from_compatibility_jamo(c: char) -> Result<Self, String> {
        match c {
            // Singular consonants
            'ㄱ' => Ok(Jamo::Consonant(JamoConsonantSingular::Giyeok)),
            'ㄴ' => Ok(Jamo::Consonant(JamoConsonantSingular::Nieun)),
            'ㄷ' => Ok(Jamo::Consonant(JamoConsonantSingular::Digeut)),
            'ㄹ' => Ok(Jamo::Consonant(JamoConsonantSingular::Rieul)),
            'ㅁ' => Ok(Jamo::Consonant(JamoConsonantSingular::Mieum)),
            'ㅂ' => Ok(Jamo::Consonant(JamoConsonantSingular::Bieup)),
            'ㅅ' => Ok(Jamo::Consonant(JamoConsonantSingular::Siot)),
            'ㅇ' => Ok(Jamo::Consonant(JamoConsonantSingular::Ieung)),
            'ㅈ' => Ok(Jamo::Consonant(JamoConsonantSingular::Jieut)),
            'ㅊ' => Ok(Jamo::Consonant(JamoConsonantSingular::Chieut)),
            'ㅋ' => Ok(Jamo::Consonant(JamoConsonantSingular::Kieuk)),
            'ㅌ' => Ok(Jamo::Consonant(JamoConsonantSingular::Tieut)),
            'ㅍ' => Ok(Jamo::Consonant(JamoConsonantSingular::Pieup)),
            'ㅎ' => Ok(Jamo::Consonant(JamoConsonantSingular::Hieut)),

            // Composite consonants
            'ㄳ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::GiyeokSiot)),
            'ㄵ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::NieunJieut)),
            'ㄶ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::NieunHieut)),
            'ㄺ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::RieulGiyeok)),
            'ㄻ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::RieulMieum)),
            'ㄼ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::RieulBieup)),
            'ㄽ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::RieulSiot)),
            'ㄾ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::RieulTieut)),
            'ㄿ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::RieulPieup)),
            'ㅀ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::RieulHieut)),
            'ㄲ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::GiyeokGiyeok)),
            'ㄸ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::DigeutDigeut)),
            'ㅃ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::BieupBieup)),
            'ㅆ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::SiotSiot)),
            'ㅉ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::JieutJieut)),
            'ㅄ' => Ok(Jamo::CompositeConsonant(JamoConsonantComposite::BieupSiot)),

            // Singular vowels
            'ㅏ' => Ok(Jamo::Vowel(JamoVowelSingular::A)),
            'ㅐ' => Ok(Jamo::Vowel(JamoVowelSingular::Ae)),
            'ㅑ' => Ok(Jamo::Vowel(JamoVowelSingular::Ya)),
            'ㅒ' => Ok(Jamo::Vowel(JamoVowelSingular::Yae)),
            'ㅓ' => Ok(Jamo::Vowel(JamoVowelSingular::Eo)),
            'ㅔ' => Ok(Jamo::Vowel(JamoVowelSingular::E)),
            'ㅕ' => Ok(Jamo::Vowel(JamoVowelSingular::Yeo)),
            'ㅖ' => Ok(Jamo::Vowel(JamoVowelSingular::Ye)),
            'ㅗ' => Ok(Jamo::Vowel(JamoVowelSingular::O)),
            'ㅛ' => Ok(Jamo::Vowel(JamoVowelSingular::Yo)),
            'ㅜ' => Ok(Jamo::Vowel(JamoVowelSingular::U)),
            'ㅠ' => Ok(Jamo::Vowel(JamoVowelSingular::Yu)),
            'ㅡ' => Ok(Jamo::Vowel(JamoVowelSingular::Eu)),
            'ㅣ' => Ok(Jamo::Vowel(JamoVowelSingular::I)),

            // Composite vowels
            'ㅘ' => Ok(Jamo::CompositeVowel(JamoVowelComposite::Wa)),
            'ㅙ' => Ok(Jamo::CompositeVowel(JamoVowelComposite::Wae)),
            'ㅚ' => Ok(Jamo::CompositeVowel(JamoVowelComposite::Oe)),
            'ㅝ' => Ok(Jamo::CompositeVowel(JamoVowelComposite::Wo)),
            'ㅞ' => Ok(Jamo::CompositeVowel(JamoVowelComposite::We)),
            'ㅟ' => Ok(Jamo::CompositeVowel(JamoVowelComposite::Wi)),
            'ㅢ' => Ok(Jamo::CompositeVowel(JamoVowelComposite::Ui)),

            _ => Err(format!("Character '{}' is not a valid compatibility jamo.", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_from_char_identifies_valid_consonants_compatibility() {
        let tests = vec![
            ('ㄱ', Jamo::Consonant(JamoConsonantSingular::Giyeok)),
            ('ㄴ', Jamo::Consonant(JamoConsonantSingular::Nieun)),
            ('ㄷ', Jamo::Consonant(JamoConsonantSingular::Digeut)),
            ('ㄹ', Jamo::Consonant(JamoConsonantSingular::Rieul)),
            ('ㅁ', Jamo::Consonant(JamoConsonantSingular::Mieum)),
            ('ㅂ', Jamo::Consonant(JamoConsonantSingular::Bieup)),
            ('ㅅ', Jamo::Consonant(JamoConsonantSingular::Siot)),
            ('ㅇ', Jamo::Consonant(JamoConsonantSingular::Ieung)),
            ('ㅈ', Jamo::Consonant(JamoConsonantSingular::Jieut)),
            ('ㅊ', Jamo::Consonant(JamoConsonantSingular::Chieut)),
            ('ㅋ', Jamo::Consonant(JamoConsonantSingular::Kieuk)),
            ('ㅌ', Jamo::Consonant(JamoConsonantSingular::Tieut)),
            ('ㅍ', Jamo::Consonant(JamoConsonantSingular::Pieup)),
            ('ㅎ', Jamo::Consonant(JamoConsonantSingular::Hieut)),
        ];
        for (c, expected_jamo) in tests {
            let result = Character::from_char(c);
            assert_eq!(
                result,
                Ok(Character::Hangul(expected_jamo)),
                "Failed on consonant: {}; got result: {:?}",
                c,
                result
            )
        }
    }

    #[test]
    fn character_from_char_identifies_valid_consonants_modern() {
        let tests = vec![
            ('ᄀ', Jamo::Consonant(JamoConsonantSingular::Giyeok)),
            ('ᄂ', Jamo::Consonant(JamoConsonantSingular::Nieun)),
            ('ᄃ', Jamo::Consonant(JamoConsonantSingular::Digeut)),
            ('ᄅ', Jamo::Consonant(JamoConsonantSingular::Rieul)),
            ('ᄆ', Jamo::Consonant(JamoConsonantSingular::Mieum)),
            ('ᄇ', Jamo::Consonant(JamoConsonantSingular::Bieup)),
            ('ᄉ', Jamo::Consonant(JamoConsonantSingular::Siot)),
            ('ᄋ', Jamo::Consonant(JamoConsonantSingular::Ieung)),
            ('ᄌ', Jamo::Consonant(JamoConsonantSingular::Jieut)),
            ('ᄎ', Jamo::Consonant(JamoConsonantSingular::Chieut)),
            ('ᄏ', Jamo::Consonant(JamoConsonantSingular::Kieuk)),
            ('ᄐ', Jamo::Consonant(JamoConsonantSingular::Tieut)),
            ('ᄑ', Jamo::Consonant(JamoConsonantSingular::Pieup)),
            ('ᄒ', Jamo::Consonant(JamoConsonantSingular::Hieut)),
        ];
        for (c, expected_jamo) in tests {
            let result = Character::from_char(c);
            assert_eq!(
                result,
                Ok(Character::Hangul(expected_jamo)),
                "Failed on consonant: {}; got result: {:?}",
                c,
                result
            )
        }
    }

    #[test]
    fn character_from_char_identifies_valid_vowels_compatibility() {
        let tests = vec![
            ('ㅏ', Jamo::Vowel(JamoVowelSingular::A)),
            ('ㅐ', Jamo::Vowel(JamoVowelSingular::Ae)),
            ('ㅑ', Jamo::Vowel(JamoVowelSingular::Ya)),
            ('ㅒ', Jamo::Vowel(JamoVowelSingular::Yae)),
            ('ㅓ', Jamo::Vowel(JamoVowelSingular::Eo)),
            ('ㅔ', Jamo::Vowel(JamoVowelSingular::E)),
            ('ㅕ', Jamo::Vowel(JamoVowelSingular::Yeo)),
            ('ㅖ', Jamo::Vowel(JamoVowelSingular::Ye)),
            ('ㅗ', Jamo::Vowel(JamoVowelSingular::O)),
            ('ㅛ', Jamo::Vowel(JamoVowelSingular::Yo)),
            ('ㅜ', Jamo::Vowel(JamoVowelSingular::U)),
            ('ㅠ', Jamo::Vowel(JamoVowelSingular::Yu)),
            ('ㅡ', Jamo::Vowel(JamoVowelSingular::Eu)),
            ('ㅣ', Jamo::Vowel(JamoVowelSingular::I)),
        ];
        for (c, expected_jamo) in tests {
            let result = Character::from_char(c);
            assert_eq!(
                result,
                Ok(Character::Hangul(expected_jamo)),
                "Failed on vowel: {}; got result: {:?}",
                c,
                result
            )
        }
    }

    #[test]
    fn character_from_char_identifies_valid_vowels_modern() {
        let tests = vec![
            ('ᅡ', Jamo::Vowel(JamoVowelSingular::A)),
            ('ᅢ', Jamo::Vowel(JamoVowelSingular::Ae)),
            ('ᅣ', Jamo::Vowel(JamoVowelSingular::Ya)),
            ('ᅤ', Jamo::Vowel(JamoVowelSingular::Yae)),
            ('ᅥ', Jamo::Vowel(JamoVowelSingular::Eo)),
            ('ᅦ', Jamo::Vowel(JamoVowelSingular::E)),
            ('ᅧ', Jamo::Vowel(JamoVowelSingular::Yeo)),
            ('ᅨ', Jamo::Vowel(JamoVowelSingular::Ye)),
            ('ᅩ', Jamo::Vowel(JamoVowelSingular::O)),
            ('ᅭ', Jamo::Vowel(JamoVowelSingular::Yo)),
            ('ᅮ', Jamo::Vowel(JamoVowelSingular::U)),
            ('ᅲ', Jamo::Vowel(JamoVowelSingular::Yu)),
            ('ᅳ', Jamo::Vowel(JamoVowelSingular::Eu)),
            ('ᅵ', Jamo::Vowel(JamoVowelSingular::I)),
        ];
        for (c, expected_jamo) in tests {
            let result = Character::from_char(c);
            assert_eq!(
                result,
                Ok(Character::Hangul(expected_jamo)),
                "Failed on vowel: {}; got result: {:?}",
                c,
                result
            )
        }
    }

    #[test]
    fn character_from_char_identifies_double_initials_compatibility() {
        let tests = vec![
            ('ㄲ', Jamo::CompositeConsonant(JamoConsonantComposite::GiyeokGiyeok)),
            ('ㄸ', Jamo::CompositeConsonant(JamoConsonantComposite::DigeutDigeut)),
            ('ㅃ', Jamo::CompositeConsonant(JamoConsonantComposite::BieupBieup)),
            ('ㅆ', Jamo::CompositeConsonant(JamoConsonantComposite::SiotSiot)),
            ('ㅉ', Jamo::CompositeConsonant(JamoConsonantComposite::JieutJieut)),
        ];
        for (c, expected_jamo) in tests {
            let result = Character::from_char(c);
            assert_eq!(
                result,
                Ok(Character::Hangul(expected_jamo)),
                "Failed on double initial: {}; got result: {:?}",
                c,
                result
            )
        }
    }

    #[test]
    fn character_from_char_identifies_double_initials_modern() {
        let tests = vec![
            ('ᄁ', Jamo::CompositeConsonant(JamoConsonantComposite::GiyeokGiyeok)),
            ('ᄄ', Jamo::CompositeConsonant(JamoConsonantComposite::DigeutDigeut)),
            ('ᄈ', Jamo::CompositeConsonant(JamoConsonantComposite::BieupBieup)),
            ('ᄊ', Jamo::CompositeConsonant(JamoConsonantComposite::SiotSiot)),
            ('ᄍ', Jamo::CompositeConsonant(JamoConsonantComposite::JieutJieut)),
        ];
        for (c, expected_jamo) in tests {
            let result = Character::from_char(c);
            assert_eq!(
                result,
                Ok(Character::Hangul(expected_jamo)),
                "Failed on double initial: {}; got result: {:?}",
                c,
                result
            )
        }
    }

    #[test]
    fn character_from_char_identifies_composite_vowels_compatibility() {
        let tests = vec![
            ('ㅘ', Jamo::CompositeVowel(JamoVowelComposite::Wa)),
            ('ㅙ', Jamo::CompositeVowel(JamoVowelComposite::Wae)),
            ('ㅚ', Jamo::CompositeVowel(JamoVowelComposite::Oe)),
            ('ㅝ', Jamo::CompositeVowel(JamoVowelComposite::Wo)),
            ('ㅞ', Jamo::CompositeVowel(JamoVowelComposite::We)),
            ('ㅟ', Jamo::CompositeVowel(JamoVowelComposite::Wi)),
            ('ㅢ', Jamo::CompositeVowel(JamoVowelComposite::Ui)),
        ];
        for (c, expected_jamo) in tests {
            let result = Character::from_char(c);
            assert_eq!(
                result,
                Ok(Character::Hangul(expected_jamo)),
                "Failed on composite vowel: {}; got result: {:?}",
                c,
                result
            )
        }
    }

    #[test] fn character_from_char_identifies_composite_vowels_modern() {
        let tests = vec![
            ('ᅪ', Jamo::CompositeVowel(JamoVowelComposite::Wa)),
            ('ᅫ', Jamo::CompositeVowel(JamoVowelComposite::Wae)),
            ('ᅬ', Jamo::CompositeVowel(JamoVowelComposite::Oe)),
            ('ᅯ', Jamo::CompositeVowel(JamoVowelComposite::Wo)),
            ('ᅰ', Jamo::CompositeVowel(JamoVowelComposite::We)),
            ('ᅱ', Jamo::CompositeVowel(JamoVowelComposite::Wi)),
            ('ᅴ', Jamo::CompositeVowel(JamoVowelComposite::Ui)),
        ];
        for (c, expected_jamo) in tests {
            let result = Character::from_char(c);
            assert_eq!(
                result,
                Ok(Character::Hangul(expected_jamo)),
                "Failed on composite vowel: {}; got result: {:?}",
                c,
                result
            )
        }
    }

    #[test]
    fn character_from_char_identifies_composite_finals_compatibility() {
        let tests = vec![
            ('ㄳ', Jamo::CompositeConsonant(JamoConsonantComposite::GiyeokSiot)),
            ('ㄵ', Jamo::CompositeConsonant(JamoConsonantComposite::NieunJieut)),
            ('ㄶ', Jamo::CompositeConsonant(JamoConsonantComposite::NieunHieut)),
            ('ㄺ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulGiyeok)),
            ('ㄻ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulMieum)),
            ('ㄼ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulBieup)),
            ('ㄽ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulSiot)),
            ('ㄾ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulTieut)),
            ('ㄿ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulPieup)),
            ('ㅀ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulHieut)),
        ];
        for (c, expected_jamo) in tests {
            let result = Character::from_char(c);
            assert_eq!(
                result,
                Ok(Character::Hangul(expected_jamo)),
                "Failed on composite final: {}; got result: {:?}",
                c,
                result
            )
        }
    }

    #[test]
    fn character_from_char_identifies_composite_finals_modern() {
        let tests = vec![
            ('ᆪ', Jamo::CompositeConsonant(JamoConsonantComposite::GiyeokSiot)),
            ('ᆬ', Jamo::CompositeConsonant(JamoConsonantComposite::NieunJieut)),
            ('ᆭ', Jamo::CompositeConsonant(JamoConsonantComposite::NieunHieut)),
            ('ᆰ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulGiyeok)),
            ('ᆱ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulMieum)),
            ('ᆲ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulBieup)),
            ('ᆳ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulSiot)),
            ('ᆴ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulTieut)),
            ('ᆵ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulPieup)),
            ('ᆶ', Jamo::CompositeConsonant(JamoConsonantComposite::RieulHieut)),
        ];
        for (c, expected_jamo) in tests {
            let result = Character::from_char(c);
            assert_eq!(
                result,
                Ok(Character::Hangul(expected_jamo)),
                "Failed on composite final: {}; got result: {:?}",
                c,
                result
            )
        }
    }

    #[test]
    fn character_from_char_identifies_non_hangul() {
        let non_hangul_chars = "ABCxyz123!@# ";
        for c in non_hangul_chars.chars() {
            let result = Character::from_char(c);
            assert!(
                result == Ok(Character::NonHangul(c)),
                "Failed on non-Hangul char: {}; got result: {:?}",
                c,
                result
            );
        }
    }
}
