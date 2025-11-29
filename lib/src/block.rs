use crate::jamo::*;
use std::fmt::Debug;

/// A struct representing a composed Hangul syllable block,
/// consisting of an initial character, a vowel character,
/// and an optional final character.
///
/// **API:**
/// ```rust
/// use hangul::block::HangulBlock;
///
/// let block = HangulBlock {
///     initial: 'ㄱ',
///     vowel: 'ㅏ',
///     final_optional: None,
/// };
///
/// // Convert the block to a Hangul syllable character
/// let syllable = block.to_char().unwrap();
/// assert_eq!(syllable, '가');
///
/// // Decompose the block into its constituent Jamo characters
/// assert_eq!(
///     block.decomposed().unwrap(),
///    (Some('ㄱ'), None, Some('ㅏ'), None, None, None)
/// );
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct HangulBlock {
    pub initial: char,
    pub vowel: char,
    pub final_optional: Option<char>,
}

impl HangulBlock {
    /// Converts the `HangulBlock` into a composed Hangul syllable unicode
    /// character. Assumes all chars are valid Jamo. If the block cannot be
    /// converted into a valid Hangul syllable, returns an `Err` with the
    /// problematic unicode code point.
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

    /// Decomposes the `HangulBlock` into its constituent Jamo characters.
    /// Returns a tuple containing six `Option<char>` values representing
    /// the decomposed characters:
    /// - First initial consonant
    /// - Second initial consonant (if composite)
    /// - First vowel
    /// - Second vowel (if composite)
    /// - First final consonant (if any)
    /// - Second final consonant (if composite)
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

/// Result of pushing a Jamo letter into a Hangul syllable block composer.
#[derive(Debug, PartialEq, Eq)]
pub enum BlockPushResult {
    /// The Jamo letter was successfully pushed into the block composer.
    Success,

    /// The Jamo letter could not be pushed because it would create
    /// an invalid Hangul syllable. However, the letter is a valid
    /// initial consonant to begin a new syllable block, so the caller
    /// should start a new block without popping any Jamo from this one.
    StartNewBlockNoPop,

    /// The Jamo letter could not be pushed because it would create
    /// an invalid Hangul syllable. The letter is not a valid initial
    /// consonant, so the caller should pop the last Jamo from this block
    /// and use it to start a new block.
    PopAndStartNewBlock,

    /// The Jamo letter is not valid in the context of Hangul syllable
    /// composition. For example, pushing a vowel when an initial consonant
    /// is expected.
    InvalidHangul,

    /// The Jamo letter is not valid Hangul.
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

/// A composer for a single Hangul syllable block. Used to build a block
/// by pushing and popping Jamo letters.
///
/// **API:**
/// ```rust
/// use hangul::block::{BlockComposer, BlockPushResult};
/// use hangul::jamo::Jamo;
///
/// let mut composer = BlockComposer::new();
///
/// // Push letters to form the syllable '강'
/// assert_eq!(composer.push(&Jamo::Consonant('ㄱ')), BlockPushResult::Success);
/// assert_eq!(composer.push(&Jamo::Vowel('ㅏ')), BlockPushResult::Success);
/// assert_eq!(composer.push(&Jamo::Consonant('ㅇ')), BlockPushResult::Success);
///
/// // Try to push another character that would not fit in the current block
/// assert_eq!(
///   composer.push(&Jamo::Vowel('ㅏ')),
///   BlockPushResult::PopAndStartNewBlock
/// );
///
/// // Get the composed block as a character
/// let block_char = composer.block_as_string().unwrap();
/// assert_eq!(block_char, Some('강'));
/// ```
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

/// The status of attempting to complete a Hangul syllable block.
#[derive(Debug, PartialEq, Eq)]
pub enum BlockCompletionStatus {
    /// The block is complete and can be represented as a `HangulBlock`.
    Complete(HangulBlock),

    /// The block is incomplete, but contains at least one Jamo character.
    Incomplete(char),

    /// The block is empty and contains no Jamo characters.
    Empty,
}

/// The status of popping a Jamo letter from a Hangul syllable block composer.
#[derive(Debug, PartialEq, Eq)]
pub enum BlockPopStatus {
    /// A Jamo letter was popped and the block still has letters remaining.
    PoppedAndNonEmpty(Jamo),

    /// A Jamo letter was popped and the block is now empty.
    PoppedAndEmpty(Jamo),

    /// The block is already empty; no letters to pop.
    None,
}

impl BlockComposer {
    /// Creates a new, empty `BlockComposer`.
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

    /// Tries to push a Jamo letter into the `BlockComposer`.
    /// Returns a `BlockPushResult` indicating the outcome of the operation.
    /// If the letter could not be pushed, the state of the current block will
    /// remain unchanged.
    pub fn push(&mut self, letter: &Jamo) -> BlockPushResult {
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

    /// Pops a Jamo letter from the `BlockComposer`. Returns a `BlockPopStatus`
    /// indicating the outcome of the operation, with values:
    /// - `PoppedAndNonEmpty(Jamo)`: A Jamo letter was popped and the block still has letters remaining.
    /// - `PoppedAndEmpty(Jamo)`: A Jamo letter was popped and the block is now empty.
    /// - `None`: The block is already empty; no letters to pop.
    ///
    /// **Example:**
    /// ```rust
    /// use hangul::block::{BlockComposer, BlockPopStatus};
    /// use hangul::jamo::Jamo;
    ///
    /// let mut composer = BlockComposer::new();
    /// composer.push(&Jamo::Consonant('ㄱ'));
    /// composer.push(&Jamo::Vowel('ㅏ'));
    ///
    /// assert_eq!(composer.pop(), BlockPopStatus::PoppedAndNonEmpty(Jamo::Vowel('ㅏ')));
    /// assert_eq!(composer.pop(), BlockPopStatus::PoppedAndEmpty(Jamo::Consonant('ㄱ')));
    /// assert_eq!(composer.pop(), BlockPopStatus::None);
    /// ```
    pub fn pop(&mut self) -> BlockPopStatus {
        if let Some(c) = self.final_second.take() {
            self.state = BlockCompositionState::ExpectingCompositeFinal;
            BlockPopStatus::PoppedAndNonEmpty(Jamo::Consonant(c))
        } else if let Some(c) = self.final_first.take() {
            self.state = match self.vowel_second {
                Some(_) => BlockCompositionState::ExpectingFinal,
                None => BlockCompositionState::ExpectingCompositeVowelOrFinal,
            };
            BlockPopStatus::PoppedAndNonEmpty(Jamo::Consonant(c))
        } else if let Some(c) = self.vowel_second.take() {
            self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
            BlockPopStatus::PoppedAndNonEmpty(Jamo::Vowel(c))
        } else if let Some(c) = self.vowel_first.take() {
            self.state = match self.initial_second {
                Some(_) => BlockCompositionState::ExpectingVowel,
                None => BlockCompositionState::ExpectingDoubleInitialOrVowel,
            };
            BlockPopStatus::PoppedAndNonEmpty(Jamo::Vowel(c))
        } else if let Some(c) = self.initial_second.take() {
            self.state = BlockCompositionState::ExpectingVowel;
            BlockPopStatus::PoppedAndNonEmpty(Jamo::Consonant(c))
        } else if let Some(c) = self.initial_first.take() {
            self.state = BlockCompositionState::ExpectingInitial;
            BlockPopStatus::PoppedAndEmpty(Jamo::Consonant(c))
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

    fn try_push_initial(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Consonant(c) => {
                self.initial_first = Some(*c);
                self.state = BlockCompositionState::ExpectingDoubleInitialOrVowel;
                BlockPushResult::Success
            }
            Jamo::CompositeConsonant(c) => {
                if is_valid_double_initial(*c) {
                    self.initial_first = Some(*c);
                    self.state = BlockCompositionState::ExpectingVowel;
                    BlockPushResult::Success
                } else {
                    BlockPushResult::InvalidHangul
                }
            }
            _ => BlockPushResult::InvalidHangul,
        }
    }

    fn try_push_double_initial_or_vowel(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Consonant(c) => {
                if let Some(initial) = self.initial_first {
                    if create_composite_initial(initial, *c).is_some() {
                        self.initial_second = Some(*c);
                        self.state = BlockCompositionState::ExpectingVowel;
                        BlockPushResult::Success
                    } else {
                        BlockPushResult::InvalidHangul
                    }
                } else {
                    BlockPushResult::InvalidHangul
                }
            }
            Jamo::Vowel(c) => {
                self.vowel_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
                BlockPushResult::Success
            }
            Jamo::CompositeVowel(c) => {
                if let Some((v1, v2)) = decompose_composite_vowel(*c) {
                    self.vowel_first = Some(v1);
                    self.vowel_second = Some(v2);
                    self.state = BlockCompositionState::ExpectingFinal;
                    BlockPushResult::Success
                } else {
                    BlockPushResult::InvalidHangul
                }
            }
            Jamo::CompositeConsonant(_) => BlockPushResult::InvalidHangul,
        }
    }

    fn try_push_vowel(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Vowel(c) => {
                self.vowel_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
                BlockPushResult::Success
            }
            Jamo::CompositeVowel(c) => {
                if let Some((v1, v2)) = decompose_composite_vowel(*c) {
                    self.vowel_first = Some(v1);
                    self.vowel_second = Some(v2);
                    self.state = BlockCompositionState::ExpectingFinal;
                    BlockPushResult::Success
                } else {
                    BlockPushResult::InvalidHangul
                }
            }
            _ => BlockPushResult::InvalidHangul,
        }
    }

    fn try_push_composite_vowel_or_final(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Vowel(c) => {
                if let Some(v1) = self.vowel_first {
                    if create_composite_vowel(v1, *c).is_some() {
                        self.initial_second = Some(*c);
                        self.state = BlockCompositionState::ExpectingFinal;
                        BlockPushResult::Success
                    } else {
                        BlockPushResult::InvalidHangul
                    }
                } else {
                    BlockPushResult::InvalidHangul
                }
            }
            Jamo::Consonant(c) => {
                self.final_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeFinal;
                BlockPushResult::Success
            }
            Jamo::CompositeConsonant(c) => {
                if is_valid_composite_final(*c) {
                    self.final_first = Some(*c);
                    self.state = BlockCompositionState::ExpectingNextBlock;
                    BlockPushResult::Success
                } else if is_valid_double_initial(*c) {
                    BlockPushResult::StartNewBlockNoPop
                } else {
                    BlockPushResult::InvalidHangul
                }
            }
            _ => BlockPushResult::InvalidHangul,
        }
    }

    fn try_push_final(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Consonant(c) => {
                self.final_first = Some(*c);
                self.state = BlockCompositionState::ExpectingCompositeFinal;
                BlockPushResult::Success
            }
            Jamo::CompositeConsonant(c) => {
                if is_valid_composite_final(*c) {
                    self.final_first = Some(*c);
                    self.state = BlockCompositionState::ExpectingNextBlock;
                    BlockPushResult::Success
                } else if is_valid_double_initial(*c) {
                    BlockPushResult::StartNewBlockNoPop
                } else {
                    BlockPushResult::InvalidHangul
                }
            }
            _ => BlockPushResult::InvalidHangul,
        }
    }

    fn try_push_composite_final(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Consonant(c) => {
                if let Some(f) = self.final_first {
                    if create_composite_final(f, *c).is_some() {
                        self.final_second = Some(*c);
                        self.state = BlockCompositionState::ExpectingNextBlock;
                        BlockPushResult::Success
                    } else {
                        BlockPushResult::StartNewBlockNoPop
                    }
                } else {
                    BlockPushResult::InvalidHangul
                }
            }
            Jamo::CompositeConsonant(c) => {
                if is_valid_double_initial(*c) {
                    BlockPushResult::StartNewBlockNoPop
                } else {
                    BlockPushResult::InvalidHangul
                }
            }
            Jamo::Vowel(_) | Jamo::CompositeVowel(_) => BlockPushResult::PopAndStartNewBlock,
        }
    }

    fn try_push_next_block(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Consonant(_) | Jamo::CompositeConsonant(_) => BlockPushResult::StartNewBlockNoPop,
            Jamo::Vowel(_) | Jamo::CompositeVowel(_) => BlockPushResult::PopAndStartNewBlock,
        }
    }

    /// Attempts to convert the current state of the `BlockComposer`
    /// into a complete `HangulBlock`. If the block is incomplete,
    /// it returns an `Incomplete` status with the last Jamo character
    /// added. If the block is empty, it returns an `Empty` status.
    ///
    /// **Example:**
    /// ```rust
    /// use hangul::block::{BlockComposer, BlockCompletionStatus, HangulBlock};
    /// use hangul::jamo::Jamo;
    ///
    /// let mut composer = BlockComposer::new();
    ///
    /// composer.push(&Jamo::Consonant('ㄱ'));
    ///
    /// // Attempt to complete incomplete block
    /// assert_eq!(
    ///     composer.try_as_complete_block(),
    ///     Ok(BlockCompletionStatus::Incomplete('ㄱ'))
    /// );
    ///
    /// composer.push(&Jamo::Vowel('ㅏ'));
    ///
    /// // Get the complete block now that a vowel has been added
    /// assert_eq!(
    ///    composer.try_as_complete_block(),
    ///    Ok(BlockCompletionStatus::Complete(HangulBlock {
    ///        initial: 'ㄱ',
    ///        vowel: 'ㅏ',
    ///        final_optional: None,
    ///    }))
    /// );
    /// ```
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
            (None, None) => match final_optional {
                Some(f) => Ok(BlockCompletionStatus::Incomplete(f)),
                None => Ok(BlockCompletionStatus::Empty),
            },
        }
    }

    /// Returns the composed Hangul syllable character as an `Option<char>`.
    pub fn block_as_string(&self) -> Result<Option<char>, String> {
        match self.try_as_complete_block()? {
            BlockCompletionStatus::Complete(block) => block
                .to_char()
                .map(Some)
                .map_err(|e| format!("Error converting block to char: U+{:04X}", e)),
            BlockCompletionStatus::Incomplete(c) => Ok(Some(c)),
            BlockCompletionStatus::Empty => Ok(None),
        }
    }

    /// Creates a `BlockComposer` from an existing `HangulBlock`,
    /// decomposing it into its constituent Jamo characters.
    /// Returns an error if decomposition fails.
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

/// Converts a vector of `HangulBlock` structs into a composed Hangul string.
/// Returns an `Err` if any block cannot be converted into a valid Hangul syllable.
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
        expected_final_word_state: BlockPushResult,
        expected_final_block_state: BlockCompositionState,
    }

    fn run_test_cases(cases: Vec<BlockComposerPushTestCase>) {
        for case in &cases {
            let mut composer = BlockComposer::new();
            let mut final_word_state = BlockPushResult::Success;
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
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingDoubleInitialOrVowel,
            },
            BlockComposerPushTestCase {
                input: vec![Jamo::Consonant('ㄱ'), Jamo::Consonant('ㄱ')],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingVowel,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::Consonant('ㄱ'),
                    Jamo::Consonant('ㄱ'),
                    Jamo::Vowel('ㅜ'),
                ],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingCompositeVowelOrFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::Consonant('ㄱ'),
                    Jamo::Consonant('ㄱ'),
                    Jamo::Vowel('ㅜ'),
                    Jamo::Vowel('ㅓ'),
                ],
                expected_final_word_state: BlockPushResult::Success,
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
                expected_final_word_state: BlockPushResult::Success,
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
                expected_final_word_state: BlockPushResult::Success,
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
                expected_final_word_state: BlockPushResult::PopAndStartNewBlock,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::CompositeConsonant('ㅃ'),
                    Jamo::Vowel('ㅣ'),
                    Jamo::CompositeConsonant('ㄳ'),
                ],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
            BlockComposerPushTestCase {
                input: vec![Jamo::Consonant('ㅈ'), Jamo::CompositeVowel('ㅚ')],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::CompositeConsonant('ㅉ'),
                    Jamo::CompositeVowel('ㅢ'),
                    Jamo::CompositeConsonant('ㅃ'),
                ],
                expected_final_word_state: BlockPushResult::StartNewBlockNoPop,
                expected_final_block_state: BlockCompositionState::ExpectingFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::Consonant('ㅇ'),
                    Jamo::Vowel('ㅣ'),
                    Jamo::Consonant('ㅅ'),
                    Jamo::Consonant('ㅅ'),
                ],
                expected_final_word_state: BlockPushResult::Success,
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
                expected_final_word_state: BlockPushResult::StartNewBlockNoPop,
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
                expected_final_word_state: BlockPushResult::InvalidHangul,
                expected_final_block_state: BlockCompositionState::ExpectingDoubleInitialOrVowel,
            },
            BlockComposerPushTestCase {
                input: vec![Jamo::Consonant('ㄱ'), Jamo::Vowel('ㅏ'), Jamo::Vowel('ㅏ')],
                expected_final_word_state: BlockPushResult::InvalidHangul,
                expected_final_block_state: BlockCompositionState::ExpectingCompositeVowelOrFinal,
            },
        ];
        run_test_cases(test_cases);
    }
}
