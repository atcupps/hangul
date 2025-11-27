use std::fmt::Debug;

use crate::chars::*;

#[derive(Debug)]
struct HangulWordComposer {
    prev_blocks: Vec<HangulBlock>,
    cur_block: BlockComposer,
}

#[derive(Debug, PartialEq, Eq)]
enum PushResult {
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
pub(crate) struct BlockComposer {
    pub(crate) state: BlockCompositionState,
    initial_first: Option<char>,
    initial_second: Option<char>,
    vowel_first: Option<char>,
    vowel_second: Option<char>,
    final_first: Option<char>,
    final_second: Option<char>,
}

impl BlockComposer {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn push(&mut self, letter: &HangulLetter) -> PushResult {
        match self.state {
            BlockCompositionState::ExpectingInitial => self.try_push_initial(letter),
            BlockCompositionState::ExpectingDoubleInitialOrVowel => self.try_push_double_initial_or_vowel(letter),
            BlockCompositionState::ExpectingVowel => self.try_push_vowel(letter),
            BlockCompositionState::ExpectingCompositeVowelOrFinal => self.try_push_composite_vowel_or_final(letter),
            BlockCompositionState::ExpectingFinal => self.try_push_final(letter),
            BlockCompositionState::ExpectingCompositeFinal => self.try_push_composite_final(letter),
            BlockCompositionState::ExpectingNextBlock => self.try_push_next_block(letter),
        }
    }

    pub(crate) fn pop_end_consonant(&mut self) -> Option<HangulLetter> {
        if let Some(c) = self.final_second.take() {
            Some(HangulLetter::Consonant(c))
        }
        else if let Some(c) = self.final_first.take() {
            Some(HangulLetter::Consonant(c))
        } 
        else {
            None
        }
    }

    fn try_push_initial(&mut self, letter: &HangulLetter) -> PushResult {
        match letter {
            HangulLetter::Consonant(c) => {
                self.initial_first = Some(*c);
                self.state = BlockCompositionState::ExpectingDoubleInitialOrVowel;
                PushResult::Success
            },
            HangulLetter::CompositeConsonant(c) => {
                if is_valid_double_initial(*c) {
                    self.initial_first = Some(*c);
                    self.state = BlockCompositionState::ExpectingVowel;
                    PushResult::Success
                } else {
                    PushResult::InvalidHangul
                }
            },
            _ => PushResult::InvalidHangul,
        }
    }

    fn try_push_double_initial_or_vowel(&mut self, letter: &HangulLetter) -> PushResult {
        match letter {
            HangulLetter::Consonant(c) => {
                if let Some(initial) = self.initial_first {
                    if consonant_doubles(initial, *c).is_some() {
                        self.initial_second = Some(*c);
                        self.state = BlockCompositionState::ExpectingVowel;
                        PushResult::Success
                    } else {
                        PushResult::InvalidHangul
                    }
                } else {
                    PushResult::InvalidHangul
                }
            },
            HangulLetter::Vowel(c) => {
                self.vowel_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
                PushResult::Success
            },
            HangulLetter::CompositeVowel(c) => {
                if let Some((v1, v2)) = decompose_composite_vowel(*c) {
                    self.vowel_first = Some(v1);
                    self.vowel_second = Some(v2);
                    self.state = BlockCompositionState::ExpectingFinal;
                    PushResult::Success
                } else {
                    PushResult::InvalidHangul
                }
            },
            HangulLetter::CompositeConsonant(_) =>
                PushResult::InvalidHangul,
        }
    }

    fn try_push_vowel(&mut self, letter: &HangulLetter) -> PushResult {
        match letter {
            HangulLetter::Vowel(c) => {
                self.vowel_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
                PushResult::Success
            },
            HangulLetter::CompositeVowel(c) => {
                if let Some((v1, v2)) = decompose_composite_vowel(*c) {
                    self.vowel_first = Some(v1);
                    self.vowel_second = Some(v2);
                    self.state = BlockCompositionState::ExpectingFinal;
                    PushResult::Success
                } else {
                    PushResult::InvalidHangul
                }
            },
            _ => PushResult::InvalidHangul,
        }
    }

    fn try_push_composite_vowel_or_final(&mut self, letter: &HangulLetter) -> PushResult {
        match letter {
            HangulLetter::Vowel(c) => {
                if let Some(v1) = self.vowel_first {
                    if composite_vowel(v1, *c).is_some() {
                        self.initial_second = Some(*c);
                        self.state = BlockCompositionState::ExpectingFinal;
                        PushResult::Success
                    } else {
                        PushResult::InvalidHangul
                    }
                } else {
                    PushResult::InvalidHangul
                }
            },
            HangulLetter::Consonant(c) => {
                self.final_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeFinal;
                PushResult::Success
            },
            HangulLetter::CompositeConsonant(c) => {
                if is_valid_composite_final(*c) {
                    self.final_first = Some(*c);
                    self.state = BlockCompositionState::ExpectingNextBlock;
                    PushResult::Success
                } else if is_valid_double_initial(*c) {
                    PushResult::StartNewBlockNoPop
                } else {
                    PushResult::InvalidHangul
                }
            },
            _ => PushResult::InvalidHangul,
        }
    }

    fn try_push_final(&mut self, letter: &HangulLetter) -> PushResult {
        match letter {
            HangulLetter::Consonant(c) => {
                self.final_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeFinal;
                PushResult::Success
            },
            HangulLetter::CompositeConsonant(c) => {
                if is_valid_composite_final(*c) {
                    self.final_first = Some(*c);
                    self.state = BlockCompositionState::ExpectingNextBlock;
                    PushResult::Success
                } else if is_valid_double_initial(*c) {
                    PushResult::StartNewBlockNoPop
                } else {
                    PushResult::InvalidHangul
                }
            },
            _ => PushResult::InvalidHangul,
        }
    }

    fn try_push_composite_final(&mut self, letter: &HangulLetter) -> PushResult {
        match letter {
            HangulLetter::Consonant(c) => {
                if let Some(f) = self.final_first {
                    if composite_final(f, *c).is_some() {
                        self.final_second = Some(*c);
                        self.state = BlockCompositionState::ExpectingNextBlock;
                        PushResult::Success
                    } else {
                        PushResult::StartNewBlockNoPop
                    }
                } else {
                    PushResult::InvalidHangul
                }
            },
            HangulLetter::CompositeConsonant(c) => {
                if is_valid_double_initial(*c) {
                    PushResult::StartNewBlockNoPop
                } else {
                    PushResult::InvalidHangul
                }
            },
            HangulLetter::Vowel(_) | HangulLetter::CompositeVowel(_) => PushResult::PopAndStartNewBlock,
        }
    }

    fn try_push_next_block(&mut self, letter: &HangulLetter) -> PushResult {
        match letter {
            HangulLetter::Consonant(_) | HangulLetter::CompositeConsonant(_) => PushResult::StartNewBlockNoPop,
            HangulLetter::Vowel(_) | HangulLetter::CompositeVowel(_) => PushResult::PopAndStartNewBlock,
        }
    }

    fn as_block(&self) -> Result<Option<HangulBlock>, String> {
        let initial = match (self.initial_first, self.initial_second) {
            (Some(i1), Some(i2)) => consonant_doubles(i1, i2).ok_or_else(|| format!("Invalid double initial consonant: {}{}", i1, i2))?,
            (Some(i1), None) => i1,
            _ => return Ok(None),
        };
        let vowel = match (self.vowel_first, self.vowel_second) {
            (Some(v1), Some(v2)) => composite_vowel(v1, v2).ok_or_else(|| format!("Invalid composite vowel: {}{}", v1, v2))?,
            (Some(v1), None) => v1,
            _ => return Ok(None),
        };
        let final_optional = match (self.final_first, self.final_second) {
            (Some(f1), Some(f2)) => Some(composite_final(f1, f2).ok_or_else(|| format!("Invalid composite final consonant: {}{}", f1, f2))?),
            (Some(f1), None) => Some(f1),
            _ => None,
        };

        let block = HangulBlock {
            initial,
            vowel,
            final_optional,
        };
        Ok(Some(block))
    }

    fn block_as_string(&self) -> Result<Option<char>, String> {
        match self.as_block()? {
            None => Ok(None),
            Some(block) => block.to_char().map(Some).map_err(|e| format!("Error converting block to char: U+{:04X}", e))
        }
    }
}

impl HangulWordComposer {
    pub fn new_word() -> Self {
        HangulWordComposer {
            prev_blocks: Vec::new(),
            cur_block: BlockComposer::new(),
        }
    }

    pub fn push_char(&mut self, c: char) -> PushResult {
        match determine_hangul(c) {
            Letter::Hangul(hl) => self.push(&hl),
            Letter::NonHangul(_) => PushResult::NonHangul,
        }
    }

    pub fn push(&mut self, letter: &HangulLetter) -> PushResult {
        self.cur_block.push(letter)
    }

    pub fn pop_and_start_new_block(&mut self, letter: HangulLetter) -> Result<(), String> {
        match self.cur_block.pop_end_consonant() {
            Some(l) => {
                self.complete_current_block()?;
                self.cur_block.push(&l);
                match self.cur_block.push(&letter) {
                    PushResult::Success => Ok(()),
                    _ => Err(format!("Error starting new block with letter: {:?}", letter)),
                }
            },
            None => Err(format!("Could not pop end consonant in function start_new_block with letter: {:?}", letter)),
        }
    }

    pub fn start_new_block(&mut self, letter: HangulLetter) -> Result<(), String> {
        self.complete_current_block()?;
        match self.cur_block.push(&letter) {
            PushResult::Success => Ok(()),
            _ => Err(format!("Error starting new block with letter: {:?}", letter)),
        }
    }

    pub fn as_string(&self) -> Result<String, String> {
        let mut result = hangul_blocks_vec_to_string(&self.prev_blocks)?;
        let cur_as_char = self.cur_block.block_as_string()?;
        if let Some(c) = cur_as_char {
            result.push(c);
        }
        Ok(result)
    }

    fn complete_current_block(&mut self) -> Result<(), String> {
        match self.cur_block.as_block()? {
            Some(block) => {
                self.prev_blocks.push(block);
                self.cur_block = BlockComposer::new();
                Ok(())
            }
            None => Err("Cannot complete current block: incomplete block state".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct HangulWordComposerPushLetterTestCase {
        input: Vec<HangulLetter>,
        expected_final_word_state: PushResult,
        expected_final_block_state: BlockCompositionState,
        expected_prev_blocks: Vec<HangulBlock>,
    }

    fn run_test_cases(cases: Vec<HangulWordComposerPushLetterTestCase>) {
        for case in &cases {
            let mut composer = HangulWordComposer::new_word();
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
                composer.cur_block.state, case.expected_final_block_state,
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
                    expected_final_word_state: PushResult::Success,
                    expected_final_block_state:
                        BlockCompositionState::ExpectingDoubleInitialOrVowel,
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![HangulLetter::Consonant('ㄱ'), HangulLetter::Consonant('ㄱ')],
                    expected_final_word_state: PushResult::Success,
                    expected_final_block_state: BlockCompositionState::ExpectingVowel,
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅜ'),
                    ],
                    expected_final_word_state: PushResult::Success,
                    expected_final_block_state:
                        BlockCompositionState::ExpectingCompositeVowelOrFinal,
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅜ'),
                        HangulLetter::Vowel('ㅓ'),
                    ],
                    expected_final_word_state: PushResult::Success,
                    expected_final_block_state: BlockCompositionState::ExpectingFinal,
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
                    expected_final_word_state: PushResult::Success,
                    expected_final_block_state:
                        BlockCompositionState::ExpectingCompositeFinal,
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
                    expected_final_word_state: PushResult::Success,
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
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
                    expected_final_word_state: PushResult::PopAndStartNewBlock,
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::CompositeConsonant('ㅃ'),
                        HangulLetter::Vowel('ㅣ'),
                        HangulLetter::CompositeConsonant('ㄳ'),
                    ],
                    expected_final_word_state: PushResult::Success,
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㅈ'),
                        HangulLetter::CompositeVowel('ㅚ'),
                    ],
                    expected_final_word_state: PushResult::Success,
                    expected_final_block_state: BlockCompositionState::ExpectingFinal,
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::CompositeConsonant('ㅉ'),
                        HangulLetter::CompositeVowel('ㅢ'),
                        HangulLetter::CompositeConsonant('ㅃ'),
                    ],
                    expected_final_word_state: PushResult::StartNewBlockNoPop,
                    expected_final_block_state: BlockCompositionState::ExpectingFinal,
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㅇ'),
                        HangulLetter::Vowel('ㅣ'),
                        HangulLetter::Consonant('ㅅ'),
                        HangulLetter::Consonant('ㅅ'),
                    ],
                    expected_final_word_state: PushResult::Success,
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
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
                    expected_final_word_state: PushResult::StartNewBlockNoPop,
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
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
                    expected_final_word_state: PushResult::InvalidHangul,
                    expected_final_block_state:
                        BlockCompositionState::ExpectingDoubleInitialOrVowel,
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerPushLetterTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅏ'),
                        HangulLetter::Vowel('ㅏ'),
                    ],
                    expected_final_word_state: PushResult::InvalidHangul,
                    expected_final_block_state:
                        BlockCompositionState::ExpectingCompositeVowelOrFinal,
                    expected_prev_blocks: vec![],
                },
            ];
        run_test_cases(test_cases);
    }

    #[test]
    fn start_new_block_valid() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.push(&HangulLetter::Consonant('ㄱ')),
            PushResult::Success
        );
        assert_eq!(
            composer.push(&HangulLetter::Vowel('ㅏ')),
            PushResult::Success
        );
        assert_eq!(
            composer.push(&HangulLetter::Consonant('ㄴ')),
            PushResult::Success,
        );
        assert_eq!(
            composer.push(&HangulLetter::Consonant('ㅇ')),
            PushResult::StartNewBlockNoPop,
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
            composer.cur_block.state,
            BlockCompositionState::ExpectingDoubleInitialOrVowel
        );
        assert_eq!(
            composer.push(&HangulLetter::Vowel('ㅛ')),
            PushResult::Success
        );
        assert_eq!(
            composer.push(&HangulLetter::CompositeConsonant('ㅉ')),
            PushResult::StartNewBlockNoPop,
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
            Err("Cannot complete current block: incomplete block state".to_string())
        );
        let _ = composer.push(&HangulLetter::Consonant('ㄱ'));
        assert_eq!(
            composer.start_new_block(HangulLetter::CompositeVowel('ㅘ')),
            Err("Cannot complete current block: incomplete block state".to_string()) 
        );
    }

    #[test]
    fn push_char_valid() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.push_char('ㄱ'),
            PushResult::Success
        );
        assert_eq!(
            composer.push_char('ㅏ'),
            PushResult::Success
        );
        assert_eq!(
            composer.push_char('ㄴ'),
            PushResult::Success,
        );
    }

    #[test]
    fn push_char_invalid_hangul() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.push_char('ㄱ'),
            PushResult::Success
        );
        assert_eq!(
            composer.push_char('ㅏ'),
            PushResult::Success
        );
        assert_eq!(
            composer.push_char('ㄹ'),
            PushResult::Success,
        );
        assert_eq!(
            composer.push_char('ㄽ'),
            PushResult::InvalidHangul,
        );
    }

    #[test]
    fn push_char_next_block() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.push_char('ㄱ'),
            PushResult::Success
        );
        assert_eq!(
            composer.push_char('ㅏ'),
            PushResult::Success
        );
        assert_eq!(
            composer.push_char('ㄴ'),
            PushResult::Success,
        );
        assert_eq!(
            composer.push_char('ㅇ'),
            PushResult::StartNewBlockNoPop,
        );
    }

    #[test]
    fn push_char_non_hangul() {
        let mut composer = HangulWordComposer::new_word();

        assert_eq!(
            composer.push_char('ㄱ'),
            PushResult::Success
        );
        assert_eq!(
            composer.push_char('ㅏ'),
            PushResult::Success
        );
        assert_eq!(
            composer.push_char('A'),
            PushResult::NonHangul,
        );
    }

    #[test]
    fn test_single_word_안녕하세요_as_string() {
        let mut composer = HangulWordComposer::new_word();
        
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::StartNewBlockNoPop);
        assert_eq!(composer.start_new_block(HangulLetter::Consonant('ㄴ')), Ok(()));
        assert_eq!(composer.push_char('ㅕ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅎ'), PushResult::StartNewBlockNoPop);
        assert_eq!(composer.start_new_block(HangulLetter::Consonant('ㅎ')), Ok(()));
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅅ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅔ'), PushResult::PopAndStartNewBlock);
        assert_eq!(composer.pop_and_start_new_block(HangulLetter::Vowel('ㅔ')), Ok(()));
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅛ'), PushResult::PopAndStartNewBlock);
        assert_eq!(composer.pop_and_start_new_block(HangulLetter::Vowel('ㅛ')), Ok(()));

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "안녕하세요".to_string());
    }

    #[test]
    fn test_single_word_앖어요_as_string() {
        let mut composer = HangulWordComposer::new_word();
        
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅓ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅂ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅅ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅇ'), PushResult::StartNewBlockNoPop);
        assert_eq!(composer.start_new_block(HangulLetter::Consonant('ㅇ')), Ok(()));
        assert_eq!(composer.push_char('ㅓ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅛ'), PushResult::PopAndStartNewBlock);
        assert_eq!(composer.pop_and_start_new_block(HangulLetter::Vowel('ㅛ')), Ok(()));

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "없어요".to_string());
    }
}
