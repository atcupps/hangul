use std::fmt::Debug;

use crate::chars::*;

#[derive(Debug)]
struct HangulWordComposer {
    prev_blocks: Vec<HangulBlock>,
    cur_block: BlockCompositionState,
}

#[derive(Debug, PartialEq, Eq)]
enum BlockCompositionState {
    /// nothing, waiting for first consonant
    ExpectingInitial,

    /// ex. ㄷ -> ㄸ or 다
    ExpectingDoubleInitialOrVowel(char),

    /// ex. ㄸ -> 따
    ExpectingVowel(char),

    /// ex. 두 -> 둬 or 둔
    ExpectingCompositeVowelOrFinal(char, char),

    /// ex. 둬 -> 뒁
    ExpectingFinal(char, char),

    /// ex. 달 -> 닳 or 다래
    ExpectingCompositeFinalOrNextBlock(char, char, char),

    /// ex. 닳 -> 달하
    ExpectingNextBlock(char, char, char),
}

#[derive(Debug, PartialEq, Eq)]
enum WordCompositionState {
    Composable,
    StartNewBlock(HangulLetter),
    InvalidHangul(HangulLetter),
    NonHangul(char),
}

impl HangulWordComposer {
    pub fn new_word() -> Self {
        HangulWordComposer {
            prev_blocks: Vec::new(),
            cur_block: BlockCompositionState::ExpectingInitial,
        }
    }

    pub fn push_char(&mut self, c: char) -> WordCompositionState {
        match determine_hangul(c) {
            Letter::Hangul(hl) => self.push(hl),
            Letter::NonHangul(ch) => WordCompositionState::NonHangul(ch),
        }
    }

    pub fn push(&mut self, letter: HangulLetter) -> WordCompositionState {
        match letter {
            HangulLetter::Consonant(c) => self.push_consonant(c),
            HangulLetter::CompositeConsonant(c) => self.push_composite_consonant(c),
            HangulLetter::Vowel(c) => self.push_vowel(c),
            HangulLetter::CompositeVowel(c) => self.push_composite_vowel(c),
        }
    }

    pub fn start_new_block(&mut self, letter: HangulLetter) -> Result<(), String> {
        match letter {
            HangulLetter::Consonant(c) => {
                self.complete_current_block()?;
                self.cur_block = BlockCompositionState::ExpectingDoubleInitialOrVowel(c);
                Ok(())
            },
            HangulLetter::CompositeConsonant(c) => {
                if is_valid_double_initial(c) {
                    self.complete_current_block()?;
                    self.cur_block = BlockCompositionState::ExpectingVowel(c);
                    Ok(())
                } else {
                    Err(format!("Cannot start new block with invalid initial consonant: {:?}", letter))
                }
            }
            _ => Err(format!("Cannot start new block with non-consonant letter: {:?}", letter)),
        }
    }

    pub fn as_string(&self) -> Result<String, String> {
        let mut result = hangul_blocks_vec_to_string(&self.prev_blocks)?;
        let cur_as_char = self.current_block_to_string()?;
        if let Some(c) = cur_as_char {
            result.push(c);
        }
        Ok(result)
    }

    fn current_block_to_string(&self) -> Result<Option<char>, String> {
        match &self.cur_block {
            BlockCompositionState::ExpectingInitial => {
                Ok(None)
            },
            BlockCompositionState::ExpectingDoubleInitialOrVowel(i) | BlockCompositionState::ExpectingVowel(i) => {
                Ok(Some(*i))
            },
            BlockCompositionState::ExpectingCompositeVowelOrFinal(i, v) | BlockCompositionState::ExpectingFinal(i, v) => {
                let block = HangulBlock {
                    initial: *i,
                    vowel: *v,
                    final_optional: None,
                };
                match block.to_char() {
                    Ok(block_char) => Ok(Some(block_char)),
                    Err(e) => Err(format!("Error converting block to char; got U+{:X}", e)),
                }
            },
            BlockCompositionState::ExpectingCompositeFinalOrNextBlock(i, v, f) | BlockCompositionState::ExpectingNextBlock(i, v, f) => {
                let block = HangulBlock {
                    initial: *i,
                    vowel: *v,
                    final_optional: Some(*f),
                };
                match block.to_char() {
                    Ok(block_char) => Ok(Some(block_char)),
                    Err(e) => Err(format!("Error converting block to char; got U+{:X}", e)),
                }
            }
        }
    }

    fn complete_current_block(&mut self) -> Result<(), String> {
        let (i, v, f) = match &self.cur_block {
            BlockCompositionState::ExpectingNextBlock(i, v, f) => (Some(*i), Some(*v), Some(*f)),
            BlockCompositionState::ExpectingCompositeFinalOrNextBlock(i, v, f) => {
                (Some(*i), Some(*v), Some(*f))
            }
            BlockCompositionState::ExpectingFinal(i, v) => (Some(*i), Some(*v), None),
            BlockCompositionState::ExpectingCompositeVowelOrFinal(i, v) => {
                (Some(*i), Some(*v), None)
            }
            BlockCompositionState::ExpectingVowel(i) => (Some(*i), None, None),
            BlockCompositionState::ExpectingDoubleInitialOrVowel(i) => (Some(*i), None, None),
            BlockCompositionState::ExpectingInitial => (None, None, None),
        };
        if let (Some(initial), Some(vowel)) = (i, v) {
            let block = HangulBlock {
                initial,
                vowel,
                final_optional: f,
            };
            self.prev_blocks.push(block);
            Ok(())
        } else {
            Err("Cannot complete current block: incomplete block state".to_string())
        }
    }

    fn push_consonant(&mut self, c: char) -> WordCompositionState {
        match self.cur_block {
            // First letter: accept initial consonant
            BlockCompositionState::ExpectingInitial => {
                self.cur_block = BlockCompositionState::ExpectingDoubleInitialOrVowel(c);
                WordCompositionState::Composable
            }

            // Second letter: try to make double consonant, else invalid
            BlockCompositionState::ExpectingDoubleInitialOrVowel(initial) => {
                if let Some(double) = consonant_doubles(initial, c) {
                    self.cur_block = BlockCompositionState::ExpectingVowel(double);
                    WordCompositionState::Composable
                } else {
                    WordCompositionState::InvalidHangul(HangulLetter::Consonant(c))
                }
            }

            // already has a double initial consonant and needs a vowel
            BlockCompositionState::ExpectingVowel(_) => WordCompositionState::InvalidHangul(HangulLetter::Consonant(c)),

            // Has a vowel, accepts a consonant as a final consonant
            BlockCompositionState::ExpectingCompositeVowelOrFinal(i, v) => {
                self.cur_block = BlockCompositionState::ExpectingCompositeFinalOrNextBlock(i, v, c);
                WordCompositionState::Composable
            }

            // Has a vowel, accepts a consonant as a final consonant
            BlockCompositionState::ExpectingFinal(i, v) => {
                self.cur_block = BlockCompositionState::ExpectingCompositeFinalOrNextBlock(i, v, c);
                WordCompositionState::Composable
            }

            // Has a final consonant; try to make composite final,
            // otherwise start a new block
            BlockCompositionState::ExpectingCompositeFinalOrNextBlock(i, v, f) => {
                if let Some(composite) = composite_final(f, c) {
                    self.cur_block = BlockCompositionState::ExpectingNextBlock(i, v, composite);
                    WordCompositionState::Composable
                } else {
                    WordCompositionState::StartNewBlock(HangulLetter::Consonant(c))
                }
            }

            // Has a composite final consonant; next consonant starts a new block.
            BlockCompositionState::ExpectingNextBlock(_, _, _) => {
                WordCompositionState::StartNewBlock(HangulLetter::Consonant(c))
            }
        }
    }

    fn push_composite_consonant(&mut self, c: char) -> WordCompositionState {
        match self.cur_block {
            // First letter: must be an initial consonant, then it's accepted,
            // and a vowel is expected next. Otherwise invalid.
            BlockCompositionState::ExpectingInitial => {
                if is_valid_double_initial(c) {
                    self.cur_block = BlockCompositionState::ExpectingVowel(c);
                    WordCompositionState::Composable
                } else {
                    WordCompositionState::InvalidHangul(HangulLetter::CompositeConsonant(c))
                }
            }

            // Final letter could be a composite consonant, but not all
            // composite consonants are valid finals to a block. If it's not
            // valid, then it could be the start of a new block if it's a valid
            // initial consonant.
            BlockCompositionState::ExpectingCompositeVowelOrFinal(i, v) => {
                if is_valid_composite_final(c) {
                    self.cur_block = BlockCompositionState::ExpectingNextBlock(i, v, c);
                    WordCompositionState::Composable
                } else if is_valid_double_initial(c) {
                    WordCompositionState::StartNewBlock(HangulLetter::CompositeConsonant(c))
                } else {
                    WordCompositionState::InvalidHangul(HangulLetter::CompositeConsonant(c))
                }
            }

            // Final letter could be a composite consonant, but not all
            // composite consonants are valid finals to a block. If it's not,
            // it could be the start of a new block if it's a valid initial.
            BlockCompositionState::ExpectingFinal(i, v) => {
                if is_valid_composite_final(c) {
                    self.cur_block = BlockCompositionState::ExpectingNextBlock(i, v, c);
                    WordCompositionState::Composable
                } else if is_valid_double_initial(c) {
                    WordCompositionState::StartNewBlock(HangulLetter::CompositeConsonant(c))
                } else {
                    WordCompositionState::InvalidHangul(HangulLetter::CompositeConsonant(c))
                }
            }

            // If there is already a final consonant, then a composite consonant
            // indicates the start of a new block using that consonant, provided
            // it is a valid initial consonant.
            BlockCompositionState::ExpectingNextBlock(_, _, _) => {
                if is_valid_double_initial(c) {
                    WordCompositionState::StartNewBlock(HangulLetter::CompositeConsonant(c))
                } else {
                    WordCompositionState::InvalidHangul(HangulLetter::CompositeConsonant(c))
                }
            }

            _ => WordCompositionState::InvalidHangul(HangulLetter::CompositeConsonant(c)),
        }
    }

    fn push_vowel(&mut self, c: char) -> WordCompositionState {
        match self.cur_block {
            // First letter must be a consonant
            BlockCompositionState::ExpectingInitial => WordCompositionState::InvalidHangul(HangulLetter::Vowel(c)),

            // Second letter: a vowel is accepted
            BlockCompositionState::ExpectingDoubleInitialOrVowel(i) => {
                self.cur_block = BlockCompositionState::ExpectingCompositeVowelOrFinal(i, c);
                WordCompositionState::Composable
            }

            // expecting vowel, accepts vowel
            BlockCompositionState::ExpectingVowel(i) => {
                self.cur_block = BlockCompositionState::ExpectingCompositeVowelOrFinal(i, c);
                WordCompositionState::Composable
            }

            // already has a vowel; try to make a composite vowel, if not valid
            // then this is an invalid state
            BlockCompositionState::ExpectingCompositeVowelOrFinal(i, v) => {
                if let Some(composite) = composite_vowel(v, c) {
                    self.cur_block = BlockCompositionState::ExpectingFinal(i, composite);
                    WordCompositionState::Composable
                } else {
                    WordCompositionState::InvalidHangul(HangulLetter::Vowel(c))
                }
            }

            // already has composite vowel, cannot accept a third
            BlockCompositionState::ExpectingFinal(_, _) => WordCompositionState::InvalidHangul(HangulLetter::Vowel(c)),

            // has a final consonant; a vowel indicates that this consonant is part of a new block
            BlockCompositionState::ExpectingCompositeFinalOrNextBlock(_, _, _) => {
                WordCompositionState::StartNewBlock(HangulLetter::Vowel(c))
            }

            // Has a composite final consonant; a vowel starts a new block
            // with the end consonant as the initial of the new block.
            BlockCompositionState::ExpectingNextBlock(_, _, _) => {
                WordCompositionState::StartNewBlock(HangulLetter::Vowel(c))
            }
        }
    }

    fn push_composite_vowel(&mut self, c: char) -> WordCompositionState {
        match self.cur_block {
            // If there is already a first letter and no vowel, then a double
            // vowel is an acceptable input.
            BlockCompositionState::ExpectingDoubleInitialOrVowel(i) => {
                self.cur_block = BlockCompositionState::ExpectingFinal(i, c);
                WordCompositionState::Composable
            }

            // If there is already a first letter and no vowel, then a double
            // vowel is an acceptable input.
            BlockCompositionState::ExpectingVowel(i) => {
                self.cur_block = BlockCompositionState::ExpectingFinal(i, c);
                WordCompositionState::Composable
            }

            // If there is a final consonant already, then a composite vowel
            // indicates the start of a new block using that consonant
            BlockCompositionState::ExpectingCompositeFinalOrNextBlock(_, _, _) => {
                WordCompositionState::StartNewBlock(HangulLetter::CompositeVowel(c))
            }

            // If there is a composite final consonant already, then a composite
            // vowel indicates the start of a new block using that consonant.
            BlockCompositionState::ExpectingNextBlock(_, _, _) => {
                WordCompositionState::StartNewBlock(HangulLetter::CompositeVowel(c))
            }

            // All other states cannot accept a composite vowel
            _ => WordCompositionState::InvalidHangul(HangulLetter::CompositeVowel(c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct HangulWordComposerPushLetterTestCase {
        input: Vec<HangulLetter>,
        expected_final_word_state: WordCompositionState,
        expected_final_block_state: BlockCompositionState,
        expected_prev_blocks: Vec<HangulBlock>,
    }

    fn run_test_cases(cases: Vec<HangulWordComposerPushLetterTestCase>) {
        for case in &cases {
            let mut composer = HangulWordComposer::new_word();
            let mut final_word_state = WordCompositionState::Composable;
            for letter in &case.input {
                final_word_state = composer.push(letter.clone());
            }
            assert_eq!(
                final_word_state, case.expected_final_word_state,
                "Final WORD state did not match expected. Composer: {:?}",
                composer
            );
            assert_eq!(
                composer.cur_block, case.expected_final_block_state,
                "Final BLOCK state did not match expected. Composer: {:?}",
                composer
            );
            assert_eq!(
                composer.prev_blocks, case.expected_prev_blocks,
                "Previous blocks did not match expected.",
            );
        }
    }

    #[test]
    fn single_block_composition_valid() {
        let test_cases: Vec<HangulWordComposerPushLetterTestCase> =
            vec![
                HangulWordComposerPushLetterTestCase {
                    input: vec![HangulLetter::Consonant('ㄱ')],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state:
                        BlockCompositionState::ExpectingDoubleInitialOrVowel('ㄱ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![HangulLetter::Consonant('ㄱ'), HangulLetter::Consonant('ㄱ')],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state: BlockCompositionState::ExpectingVowel('ㄲ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅜ'),
                    ],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state:
                        BlockCompositionState::ExpectingCompositeVowelOrFinal('ㄲ', 'ㅜ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅜ'),
                        HangulLetter::Vowel('ㅓ'),
                    ],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state: BlockCompositionState::ExpectingFinal('ㄲ', 'ㅝ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅜ'),
                        HangulLetter::Vowel('ㅓ'),
                        HangulLetter::Consonant('ㄹ'),
                    ],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state:
                        BlockCompositionState::ExpectingCompositeFinalOrNextBlock('ㄲ', 'ㅝ', 'ㄹ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅜ'),
                        HangulLetter::Vowel('ㅓ'),
                        HangulLetter::Consonant('ㄹ'),
                        HangulLetter::Consonant('ㅎ'),
                    ],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock(
                        'ㄲ', 'ㅝ', 'ㅀ',
                    ),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅜ'),
                        HangulLetter::Vowel('ㅓ'),
                        HangulLetter::Consonant('ㄹ'),
                        HangulLetter::Consonant('ㅎ'),
                        HangulLetter::Vowel('ㅏ'),
                    ],
                    expected_final_word_state: WordCompositionState::StartNewBlock(HangulLetter::Vowel('ㅏ')),
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock(
                        'ㄲ', 'ㅝ', 'ㅀ',
                    ),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::CompositeConsonant('ㅃ'),
                        HangulLetter::Vowel('ㅣ'),
                        HangulLetter::CompositeConsonant('ㄳ'),
                    ],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock(
                        'ㅃ', 'ㅣ', 'ㄳ',
                    ),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㅈ'),
                        HangulLetter::CompositeVowel('ㅚ'),
                    ],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state: BlockCompositionState::ExpectingFinal('ㅈ', 'ㅚ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::CompositeConsonant('ㅉ'),
                        HangulLetter::CompositeVowel('ㅢ'),
                        HangulLetter::CompositeConsonant('ㅃ'),
                    ],
                    expected_final_word_state: WordCompositionState::StartNewBlock(HangulLetter::CompositeConsonant('ㅃ')),
                    expected_final_block_state: BlockCompositionState::ExpectingFinal('ㅉ', 'ㅢ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㅇ'),
                        HangulLetter::Vowel('ㅣ'),
                        HangulLetter::Consonant('ㅅ'),
                        HangulLetter::Consonant('ㅅ'),
                    ],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock(
                        'ㅇ', 'ㅣ', 'ㅆ',
                    ),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㅇ'),
                        HangulLetter::Vowel('ㅣ'),
                        HangulLetter::Consonant('ㅅ'),
                        HangulLetter::Consonant('ㅅ'),
                        HangulLetter::Consonant('ㅅ'),
                    ],
                    expected_final_word_state: WordCompositionState::StartNewBlock(HangulLetter::Consonant('ㅅ')),
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock(
                        'ㅇ', 'ㅣ', 'ㅆ',
                    ),
                    expected_prev_blocks: vec![],
                },
            ];

        run_test_cases(test_cases);
    }

    #[test]
    fn single_block_composition_invalid() {
        let test_cases: Vec<HangulWordComposerPushLetterTestCase> =
            vec![
                HangulWordComposerPushLetterTestCase {
                    input: vec![HangulLetter::Consonant('ㄱ'), HangulLetter::Consonant('ㄹ')],
                    expected_final_word_state: WordCompositionState::InvalidHangul(HangulLetter::Consonant('ㄹ')),
                    expected_final_block_state:
                        BlockCompositionState::ExpectingDoubleInitialOrVowel('ㄱ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅏ'),
                        HangulLetter::Vowel('ㅏ'),
                    ],
                    expected_final_word_state: WordCompositionState::InvalidHangul(HangulLetter::Vowel('ㅏ')),
                    expected_final_block_state:
                        BlockCompositionState::ExpectingCompositeVowelOrFinal('ㄱ', 'ㅏ'),
                    expected_prev_blocks: vec![],
                },
            ];
        run_test_cases(test_cases);
    }

    #[test]
    fn start_new_block_valid() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.push(HangulLetter::Consonant('ㄱ')),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push(HangulLetter::Vowel('ㅏ')),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push(HangulLetter::Consonant('ㄴ')),
            WordCompositionState::Composable,
        );
        assert_eq!(
            composer.push(HangulLetter::Consonant('ㅇ')),
            WordCompositionState::StartNewBlock(HangulLetter::Consonant('ㅇ')),
        );
        assert_eq!(
            composer.start_new_block(HangulLetter::Consonant('ㅇ')),
            Ok(())
        );
        assert_eq!(
            composer.prev_blocks,
            vec![
                HangulBlock {
                    initial: 'ㄱ',
                    vowel: 'ㅏ',
                    final_optional: Some('ㄴ'),
                }
            ]
        );
        assert_eq!(
            composer.cur_block,
            BlockCompositionState::ExpectingDoubleInitialOrVowel('ㅇ')
        );
        assert_eq!(
            composer.push(HangulLetter::Vowel('ㅛ')),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push(HangulLetter::CompositeConsonant('ㅉ')),
            WordCompositionState::StartNewBlock(HangulLetter::CompositeConsonant('ㅉ')),
        );
        assert_eq!(
            composer.start_new_block(HangulLetter::CompositeConsonant('ㅉ')),
            Ok(())
        );
        assert_eq!(
            composer.prev_blocks,
            vec![
                HangulBlock {
                    initial: 'ㄱ',
                    vowel: 'ㅏ',
                    final_optional: Some('ㄴ'),
                },
                HangulBlock {
                    initial: 'ㅇ',
                    vowel: 'ㅛ',
                    final_optional: None,
                }
            ]
        );
    }

    #[test]
    fn start_new_block_invalid() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.start_new_block(HangulLetter::Vowel('ㅏ')),
            Err("Cannot start new block with non-consonant letter: Vowel('ㅏ')".to_string())
        );
        let _ = composer.push(HangulLetter::Consonant('ㄱ'));
        assert_eq!(
            composer.start_new_block(HangulLetter::CompositeVowel('ㅘ')),
            Err("Cannot start new block with non-consonant letter: CompositeVowel('ㅘ')".to_string()) 
        );
    }

    #[test]
    fn push_char_valid() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.push_char('ㄱ'),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push_char('ㅏ'),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push_char('ㄴ'),
            WordCompositionState::Composable,
        );
    }

    #[test]
    fn push_char_invalid_hangul() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.push_char('ㄱ'),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push_char('ㅏ'),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push_char('ㄹ'),
            WordCompositionState::Composable,
        );
        assert_eq!(
            composer.push_char('ㄽ'),
            WordCompositionState::InvalidHangul(HangulLetter::CompositeConsonant('ㄽ')),
        );
    }

    #[test]
    fn push_char_next_block() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.push_char('ㄱ'),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push_char('ㅏ'),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push_char('ㄴ'),
            WordCompositionState::Composable,
        );
        assert_eq!(
            composer.push_char('ㅇ'),
            WordCompositionState::StartNewBlock(HangulLetter::Consonant('ㅇ')),
        );
    }

    #[test]
    fn push_char_non_hangul() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.push_char('ㄱ'),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push_char('ㅏ'),
            WordCompositionState::Composable
        );
        assert_eq!(
            composer.push_char('A'),
            WordCompositionState::NonHangul('A'),
        );
    }

    struct HangulWordComposerStringTestCase {
        input: Vec<char>,
        expected_output: String,
    }

    fn run_string_test_cases(cases: Vec<HangulWordComposerStringTestCase>) {
        for case in &cases {
            let mut composer = HangulWordComposer::new_word();
            for c in &case.input {
                let state = composer.push_char(*c);
                if let WordCompositionState::StartNewBlock(new_c) = state {
                    let _ = composer.start_new_block(new_c);
                }
            }
            let as_string = composer.as_string().unwrap();
            assert_eq!(
                as_string, case.expected_output,
                "Output string did not match expected. Composer: {:?}",
                composer
            );
        }
    }

    #[test]
    fn input_then_as_string() {
        let test_cases: Vec<HangulWordComposerStringTestCase> = vec![
            HangulWordComposerStringTestCase {
                input: vec!['ㅇ', 'ㅏ', 'ㄴ', 'ㄴ', 'ㅕ', 'ㅇ', 'ㅎ', 'ㅏ', 'ㅅ', 'ㅔ', 'ㅇ', 'ㅛ'],
                expected_output: "안녕하세요".to_string(),
            },
            HangulWordComposerStringTestCase {
                input: vec!['ㅇ', 'ㅓ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅓ', 'ㅇ', 'ㅛ'],
                expected_output: "없어요".to_string(),
            }
        ];
        run_string_test_cases(test_cases);
    }
}
