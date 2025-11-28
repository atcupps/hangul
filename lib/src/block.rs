use crate::jamo::*;
use std::fmt::Debug;

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

    pub fn decomposed(
        &self,
    ) -> Result<
        (
            Option<char>,
            Option<char>,
            Option<char>,
            Option<char>,
            Option<char>,
            Option<char>,
        ),
        String,
    > {
        let (i1, i2) = match decompose_composite_initial(self.initial) {
            Some((a, b)) => (Some(a), Some(b)),
            None => (Some(self.initial), None),
        };
        let (v1, v2) = match decompose_composite_vowel(self.vowel) {
            Some((a, b)) => (Some(a), Some(b)),
            None => (Some(self.vowel), None),
        };
        let (f1, f2) = match self.final_optional {
            Some(c) => match decompose_composite_final(c) {
                Some((a, b)) => (Some(a), Some(b)),
                None => (Some(c), None),
            },
            None => (None, None),
        };
        Ok((i1, i2, v1, v2, f1, f2))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PushResult {
    Success,
    StartNewBlockNoPop,
    PopAndStartNewBlock,
    InvalidHangul,
    NonHangul,
}

#[derive(Debug, PartialEq, Eq)]
enum BlockCompositionState {
    /// nothing, waiting for first consonant
    ExpectingInitial,

    /// ex. ㄷ -> ㄸ or 다
    ExpectingDoubleInitialOrVowel,

    /// ex. ㄸ -> 따
    ExpectingVowel,

    /// ex. 두 -> 둬 or 둔
    ExpectingCompositeVowelOrFinal,

    /// ex. 둬 -> 뒁
    ExpectingFinal,

    /// ex. 달 -> 닳 or 다래
    ExpectingCompositeFinal,

    /// ex. 닳 -> 달하
    ExpectingNextBlock,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockComposer {
    state: BlockCompositionState,
    initial_first: Option<char>,
    initial_second: Option<char>,
    vowel_first: Option<char>,
    vowel_second: Option<char>,
    final_first: Option<char>,
    final_second: Option<char>,
}

pub enum BlockCompletionStatus {
    Complete(HangulBlock),
    Incomplete(char),
}

pub enum BlockPopStatus {
    PoppedAndShouldContinue(Jamo),
    PoppedAndShouldRemove(Jamo),
    None,
}

impl BlockComposer {
    pub fn new() -> Self {
        BlockComposer {
            state: BlockCompositionState::ExpectingInitial,
            initial_first: None,
            initial_second: None,
            vowel_first: None,
            vowel_second: None,
            final_first: None,
            final_second: None,
        }
    }

    pub fn push(&mut self, letter: &Jamo) -> PushResult {
        match self.state {
            BlockCompositionState::ExpectingInitial => self.try_push_initial(letter),
            BlockCompositionState::ExpectingDoubleInitialOrVowel => {
                self.try_push_double_initial_or_vowel(letter)
            }
            BlockCompositionState::ExpectingVowel => self.try_push_vowel(letter),
            BlockCompositionState::ExpectingCompositeVowelOrFinal => {
                self.try_push_composite_vowel_or_final(letter)
            }
            BlockCompositionState::ExpectingFinal => self.try_push_final(letter),
            BlockCompositionState::ExpectingCompositeFinal => self.try_push_composite_final(letter),
            BlockCompositionState::ExpectingNextBlock => self.try_push_next_block(letter),
        }
    }

    pub fn pop(&mut self) -> BlockPopStatus {
        if let Some(c) = self.final_second.take() {
            self.state = BlockCompositionState::ExpectingCompositeFinal;
            BlockPopStatus::PoppedAndShouldContinue(Jamo::Consonant(c))
        } else if let Some(c) = self.final_first.take() {
            self.state = match self.vowel_second {
                Some(_) => BlockCompositionState::ExpectingFinal,
                None => BlockCompositionState::ExpectingCompositeVowelOrFinal,
            };
            BlockPopStatus::PoppedAndShouldContinue(Jamo::Consonant(c))
        } else if let Some(c) = self.vowel_second.take() {
            self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
            BlockPopStatus::PoppedAndShouldContinue(Jamo::Vowel(c))
        } else if let Some(c) = self.vowel_first.take() {
            self.state = match self.initial_second {
                Some(_) => BlockCompositionState::ExpectingVowel,
                None => BlockCompositionState::ExpectingDoubleInitialOrVowel,
            };
            BlockPopStatus::PoppedAndShouldContinue(Jamo::Vowel(c))
        } else if let Some(c) = self.initial_second.take() {
            self.state = BlockCompositionState::ExpectingVowel;
            BlockPopStatus::PoppedAndShouldContinue(Jamo::Consonant(c))
        } else if let Some(c) = self.initial_first.take() {
            self.state = BlockCompositionState::ExpectingInitial;
            BlockPopStatus::PoppedAndShouldRemove(Jamo::Consonant(c))
        } else {
            self.state = BlockCompositionState::ExpectingInitial;
            BlockPopStatus::None
        }
    }

    pub(crate) fn pop_end_consonant(&mut self) -> Option<Jamo> {
        if let Some(c) = self.final_second.take() {
            Some(Jamo::Consonant(c))
        } else if let Some(c) = self.final_first.take() {
            Some(Jamo::Consonant(c))
        } else {
            None
        }
    }

    fn try_push_initial(&mut self, letter: &Jamo) -> PushResult {
        match letter {
            Jamo::Consonant(c) => {
                self.initial_first = Some(*c);
                self.state = BlockCompositionState::ExpectingDoubleInitialOrVowel;
                PushResult::Success
            }
            Jamo::CompositeConsonant(c) => {
                if is_valid_double_initial(*c) {
                    self.initial_first = Some(*c);
                    self.state = BlockCompositionState::ExpectingVowel;
                    PushResult::Success
                } else {
                    PushResult::InvalidHangul
                }
            }
            _ => PushResult::InvalidHangul,
        }
    }

    fn try_push_double_initial_or_vowel(&mut self, letter: &Jamo) -> PushResult {
        match letter {
            Jamo::Consonant(c) => {
                if let Some(initial) = self.initial_first {
                    if create_composite_initial(initial, *c).is_some() {
                        self.initial_second = Some(*c);
                        self.state = BlockCompositionState::ExpectingVowel;
                        PushResult::Success
                    } else {
                        PushResult::InvalidHangul
                    }
                } else {
                    PushResult::InvalidHangul
                }
            }
            Jamo::Vowel(c) => {
                self.vowel_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
                PushResult::Success
            }
            Jamo::CompositeVowel(c) => {
                if let Some((v1, v2)) = decompose_composite_vowel(*c) {
                    self.vowel_first = Some(v1);
                    self.vowel_second = Some(v2);
                    self.state = BlockCompositionState::ExpectingFinal;
                    PushResult::Success
                } else {
                    PushResult::InvalidHangul
                }
            }
            Jamo::CompositeConsonant(_) => PushResult::InvalidHangul,
        }
    }

    fn try_push_vowel(&mut self, letter: &Jamo) -> PushResult {
        match letter {
            Jamo::Vowel(c) => {
                self.vowel_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
                PushResult::Success
            }
            Jamo::CompositeVowel(c) => {
                if let Some((v1, v2)) = decompose_composite_vowel(*c) {
                    self.vowel_first = Some(v1);
                    self.vowel_second = Some(v2);
                    self.state = BlockCompositionState::ExpectingFinal;
                    PushResult::Success
                } else {
                    PushResult::InvalidHangul
                }
            }
            _ => PushResult::InvalidHangul,
        }
    }

    fn try_push_composite_vowel_or_final(&mut self, letter: &Jamo) -> PushResult {
        match letter {
            Jamo::Vowel(c) => {
                if let Some(v1) = self.vowel_first {
                    if create_composite_vowel(v1, *c).is_some() {
                        self.initial_second = Some(*c);
                        self.state = BlockCompositionState::ExpectingFinal;
                        PushResult::Success
                    } else {
                        PushResult::InvalidHangul
                    }
                } else {
                    PushResult::InvalidHangul
                }
            }
            Jamo::Consonant(c) => {
                self.final_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeFinal;
                PushResult::Success
            }
            Jamo::CompositeConsonant(c) => {
                if is_valid_composite_final(*c) {
                    self.final_first = Some(*c);
                    self.state = BlockCompositionState::ExpectingNextBlock;
                    PushResult::Success
                } else if is_valid_double_initial(*c) {
                    PushResult::StartNewBlockNoPop
                } else {
                    PushResult::InvalidHangul
                }
            }
            _ => PushResult::InvalidHangul,
        }
    }

    fn try_push_final(&mut self, letter: &Jamo) -> PushResult {
        match letter {
            Jamo::Consonant(c) => {
                self.final_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeFinal;
                PushResult::Success
            }
            Jamo::CompositeConsonant(c) => {
                if is_valid_composite_final(*c) {
                    self.final_first = Some(*c);
                    self.state = BlockCompositionState::ExpectingNextBlock;
                    PushResult::Success
                } else if is_valid_double_initial(*c) {
                    PushResult::StartNewBlockNoPop
                } else {
                    PushResult::InvalidHangul
                }
            }
            _ => PushResult::InvalidHangul,
        }
    }

    fn try_push_composite_final(&mut self, letter: &Jamo) -> PushResult {
        match letter {
            Jamo::Consonant(c) => {
                if let Some(f) = self.final_first {
                    if create_composite_final(f, *c).is_some() {
                        self.final_second = Some(*c);
                        self.state = BlockCompositionState::ExpectingNextBlock;
                        PushResult::Success
                    } else {
                        PushResult::StartNewBlockNoPop
                    }
                } else {
                    PushResult::InvalidHangul
                }
            }
            Jamo::CompositeConsonant(c) => {
                if is_valid_double_initial(*c) {
                    PushResult::StartNewBlockNoPop
                } else {
                    PushResult::InvalidHangul
                }
            }
            Jamo::Vowel(_) | Jamo::CompositeVowel(_) => PushResult::PopAndStartNewBlock,
        }
    }

    fn try_push_next_block(&mut self, letter: &Jamo) -> PushResult {
        match letter {
            Jamo::Consonant(_) | Jamo::CompositeConsonant(_) => PushResult::StartNewBlockNoPop,
            Jamo::Vowel(_) | Jamo::CompositeVowel(_) => PushResult::PopAndStartNewBlock,
        }
    }

    pub fn try_as_complete_block(&self) -> Result<BlockCompletionStatus, String> {
        let initial_optional = match (self.initial_first, self.initial_second) {
            (Some(i1), Some(i2)) => Some(
                create_composite_initial(i1, i2)
                    .ok_or_else(|| format!("Invalid double initial consonant: {}{}", i1, i2))?,
            ),
            (Some(i1), None) => Some(i1),
            _ => None,
        };
        let vowel_optional = match (self.vowel_first, self.vowel_second) {
            (Some(v1), Some(v2)) => Some(
                create_composite_vowel(v1, v2)
                    .ok_or_else(|| format!("Invalid composite vowel: {}{}", v1, v2))?,
            ),
            (Some(v1), None) => Some(v1),
            _ => None,
        };
        let final_optional = match (self.final_first, self.final_second) {
            (Some(f1), Some(f2)) => Some(
                create_composite_final(f1, f2)
                    .ok_or_else(|| format!("Invalid composite final consonant: {}{}", f1, f2))?,
            ),
            (Some(f1), None) => Some(f1),
            _ => None,
        };

        match (initial_optional, vowel_optional) {
            (Some(initial), Some(vowel)) => Ok(BlockCompletionStatus::Complete(HangulBlock {
                initial,
                vowel,
                final_optional,
            })),
            (Some(initial), None) => Ok(BlockCompletionStatus::Incomplete(initial)),
            (None, Some(vowel)) => Ok(BlockCompletionStatus::Incomplete(vowel)),
            (None, None) => {
                Err("Cannot form block: missing initial consonant and vowel".to_string())
            }
        }
    }

    pub fn block_as_string(&self) -> Result<Option<char>, String> {
        match self.try_as_complete_block()? {
            BlockCompletionStatus::Complete(block) => block
                .to_char()
                .map(Some)
                .map_err(|e| format!("Error converting block to char: U+{:04X}", e)),
            BlockCompletionStatus::Incomplete(c) => Ok(Some(c)),
        }
    }

    pub fn from_composed_block(block: &HangulBlock) -> Result<Self, String> {
        let mut result = BlockComposer::new();
        let (i1, i2, v1, v2, f1, f2) = block.decomposed()?;

        if f2.is_some() {
            result.state = BlockCompositionState::ExpectingNextBlock;
        } else if f1.is_some() {
            result.state = BlockCompositionState::ExpectingCompositeFinal;
        } else if v2.is_some() {
            result.state = BlockCompositionState::ExpectingFinal;
        } else if v1.is_some() {
            result.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
        }
        // Anything after this shouldn't happen. But this won't return an error
        // because it's conceivable that a manually constructed HangulBlock
        // leads to one of these states occuring. This may lead to undefined
        // behavior.
        else if i2.is_some() {
            result.state = BlockCompositionState::ExpectingVowel;
        } else if i1.is_some() {
            result.state = BlockCompositionState::ExpectingDoubleInitialOrVowel;
        } else {
            result.state = BlockCompositionState::ExpectingInitial;
        }

        result.initial_first = i1;
        result.initial_second = i2;
        result.vowel_first = v1;
        result.vowel_second = v2;
        result.final_first = f1;
        result.final_second = f2;

        Ok(result)
    }
}

// Convert compatibility jamo to modern jamo.

pub fn hangul_blocks_vec_to_string(blocks: &Vec<HangulBlock>) -> Result<String, String> {
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
                result == Character::Hangul(Jamo::Consonant(c)),
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
                result == Character::Hangul(Jamo::Vowel(c)),
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
                result == Character::Hangul(Jamo::CompositeConsonant(c)),
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
                result == Character::Hangul(Jamo::CompositeVowel(c)),
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
                result == Character::Hangul(Jamo::CompositeConsonant(c)),
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
                result == Character::NonHangul(c),
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

    struct BlockComposerPushTestCase {
        input: Vec<Jamo>,
        expected_final_word_state: PushResult,
        expected_final_block_state: BlockCompositionState,
    }

    fn run_test_cases(cases: Vec<BlockComposerPushTestCase>) {
        for case in &cases {
            let mut composer = BlockComposer::new();
            let mut final_word_state = PushResult::Success;
            for letter in &case.input {
                final_word_state = composer.push(letter);
            }
            assert_eq!(
                final_word_state, case.expected_final_word_state,
                "Final WORD state did not match expected. Composer: {:?}",
                composer
            );
            assert_eq!(
                composer.state, case.expected_final_block_state,
                "Final BLOCK state did not match expected. Composer: {:?}",
                composer
            );
        }
    }

    #[test]
    fn single_block_composition_valid() {
        let test_cases: Vec<BlockComposerPushTestCase> = vec![
            BlockComposerPushTestCase {
                input: vec![Jamo::Consonant('ㄱ')],
                expected_final_word_state: PushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingDoubleInitialOrVowel,
            },
            BlockComposerPushTestCase {
                input: vec![Jamo::Consonant('ㄱ'), Jamo::Consonant('ㄱ')],
                expected_final_word_state: PushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingVowel,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::Consonant('ㄱ'),
                    Jamo::Consonant('ㄱ'),
                    Jamo::Vowel('ㅜ'),
                ],
                expected_final_word_state: PushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingCompositeVowelOrFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::Consonant('ㄱ'),
                    Jamo::Consonant('ㄱ'),
                    Jamo::Vowel('ㅜ'),
                    Jamo::Vowel('ㅓ'),
                ],
                expected_final_word_state: PushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::Consonant('ㄱ'),
                    Jamo::Consonant('ㄱ'),
                    Jamo::Vowel('ㅜ'),
                    Jamo::Vowel('ㅓ'),
                    Jamo::Consonant('ㄹ'),
                ],
                expected_final_word_state: PushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingCompositeFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::Consonant('ㄱ'),
                    Jamo::Consonant('ㄱ'),
                    Jamo::Vowel('ㅜ'),
                    Jamo::Vowel('ㅓ'),
                    Jamo::Consonant('ㄹ'),
                    Jamo::Consonant('ㅎ'),
                ],
                expected_final_word_state: PushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::Consonant('ㄱ'),
                    Jamo::Consonant('ㄱ'),
                    Jamo::Vowel('ㅜ'),
                    Jamo::Vowel('ㅓ'),
                    Jamo::Consonant('ㄹ'),
                    Jamo::Consonant('ㅎ'),
                    Jamo::Vowel('ㅏ'),
                ],
                expected_final_word_state: PushResult::PopAndStartNewBlock,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::CompositeConsonant('ㅃ'),
                    Jamo::Vowel('ㅣ'),
                    Jamo::CompositeConsonant('ㄳ'),
                ],
                expected_final_word_state: PushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
            BlockComposerPushTestCase {
                input: vec![Jamo::Consonant('ㅈ'), Jamo::CompositeVowel('ㅚ')],
                expected_final_word_state: PushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::CompositeConsonant('ㅉ'),
                    Jamo::CompositeVowel('ㅢ'),
                    Jamo::CompositeConsonant('ㅃ'),
                ],
                expected_final_word_state: PushResult::StartNewBlockNoPop,
                expected_final_block_state: BlockCompositionState::ExpectingFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::Consonant('ㅇ'),
                    Jamo::Vowel('ㅣ'),
                    Jamo::Consonant('ㅅ'),
                    Jamo::Consonant('ㅅ'),
                ],
                expected_final_word_state: PushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::Consonant('ㅇ'),
                    Jamo::Vowel('ㅣ'),
                    Jamo::Consonant('ㅅ'),
                    Jamo::Consonant('ㅅ'),
                    Jamo::Consonant('ㅅ'),
                ],
                expected_final_word_state: PushResult::StartNewBlockNoPop,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
        ];

        run_test_cases(test_cases);
    }

    #[test]
    fn single_block_composition_invalid() {
        let test_cases: Vec<BlockComposerPushTestCase> = vec![
            BlockComposerPushTestCase {
                input: vec![Jamo::Consonant('ㄱ'), Jamo::Consonant('ㄹ')],
                expected_final_word_state: PushResult::InvalidHangul,
                expected_final_block_state: BlockCompositionState::ExpectingDoubleInitialOrVowel,
            },
            BlockComposerPushTestCase {
                input: vec![Jamo::Consonant('ㄱ'), Jamo::Vowel('ㅏ'), Jamo::Vowel('ㅏ')],
                expected_final_word_state: PushResult::InvalidHangul,
                expected_final_block_state: BlockCompositionState::ExpectingCompositeVowelOrFinal,
            },
        ];
        run_test_cases(test_cases);
    }
}
