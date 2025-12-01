use crate::jamo::*;
use std::fmt::Debug;

/// A struct representing a composed Hangul syllable block,
/// consisting of an initial character, a vowel character,
/// and an optional final character.
///
/// **API:**
/// ```rust
/// use hangul::block::HangulBlock;
/// use hangul::jamo::{Jamo, JamoConsonantSingular, JamoVowelSingular};
///
/// let block = HangulBlock {
///     initial: Jamo::from_compatibility_jamo('ㄱ').unwrap(),
///     vowel: Jamo::from_compatibility_jamo('ㅏ').unwrap(),
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
///    (Some(Jamo::Consonant(JamoConsonantSingular::Giyeok)), None, Some(Jamo::Vowel(JamoVowelSingular::A)), None, None, None)
/// );
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct HangulBlock {
    pub initial: Jamo,
    pub vowel: Jamo,
    pub final_optional: Option<Jamo>,
}

impl HangulBlock {
    /// Converts the `HangulBlock` into a composed Hangul syllable unicode
    /// character. Assumes all chars are valid Jamo. If the block cannot be
    /// converted into a valid Hangul syllable, returns an `Err` with the
    /// problematic unicode code point.
    pub fn to_char(&self) -> Result<char, u32> {
        // Ensure the initial, vowel, and final are modern Jamo and not
        // compatibility jamo
        let initial = match self.initial.char_modern(JamoPosition::Initial) {
            Some(c) => c,
            None => return Err(0),
        };
        let vowel = match self.vowel.char_modern(JamoPosition::Vowel) {
            Some(c) => c,
            None => return Err(0),
        };
        let final_optional = match &self.final_optional {
            Some(c) => c.char_modern(JamoPosition::Final),
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
            Option<Jamo>,
            Option<Jamo>,
            Option<Jamo>,
            Option<Jamo>,
            Option<Jamo>,
            Option<Jamo>,
        ),
        String,
    > {
        let (i1, i2) = match &self.initial {
            Jamo::CompositeConsonant(c) => match c.decompose() {
                (a, b) => (Some(a), Some(b)),
            },
            Jamo::Consonant(c) => (Some(Jamo::Consonant(c.clone())), None),
            _ => (None, None),
        };

        let (v1, v2) = match &self.vowel {
            Jamo::CompositeVowel(c) => match c.decompose() {
                (a, b) => (Some(a), Some(b)),
            },
            Jamo::Vowel(c) => (Some(Jamo::Vowel(c.clone())), None),
            _ => (None, None),
        };

        let (f1, f2) = match &self.final_optional {
            Some(Jamo::CompositeConsonant(c)) => match c.decompose() {
                (a, b) => (Some(a), Some(b)),
            },
            Some(Jamo::Consonant(c)) => (Some(Jamo::Consonant(c.clone())), None),
            _ => (None, None),
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
/// use hangul::jamo::{Jamo, JamoConsonantSingular, JamoVowelSingular};
///
/// let mut composer = BlockComposer::new();
///
/// // Push letters to form the syllable '강'
/// assert_eq!(composer.push(&Jamo::Consonant(JamoConsonantSingular::Giyeok)), BlockPushResult::Success);
/// assert_eq!(composer.push(&Jamo::Vowel(JamoVowelSingular::A)), BlockPushResult::Success);
/// assert_eq!(composer.push(&Jamo::Consonant(JamoConsonantSingular::Ieung)), BlockPushResult::Success);
///
/// // Try to push another character that would not fit in the current block
/// assert_eq!(
///   composer.push(&Jamo::Vowel(JamoVowelSingular::A)),
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
    initial_first: Option<Jamo>,
    initial_second: Option<Jamo>,
    vowel_first: Option<Jamo>,
    vowel_second: Option<Jamo>,
    final_first: Option<Jamo>,
    final_second: Option<Jamo>,
}

/// The status of attempting to complete a Hangul syllable block.
#[derive(Debug, PartialEq, Eq)]
pub enum BlockCompletionStatus {
    /// The block is complete and can be represented as a `HangulBlock`.
    Complete(HangulBlock),

    /// The block is incomplete, but contains at one Jamo character.
    Incomplete(Jamo),

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
    /// use hangul::jamo::{Jamo, JamoConsonantSingular, JamoVowelSingular};
    ///
    /// let mut composer = BlockComposer::new();
    /// composer.push(&Jamo::from_compatibility_jamo('ㄱ').unwrap());
    /// composer.push(&Jamo::from_compatibility_jamo('ㅏ').unwrap());
    ///
    /// assert_eq!(composer.pop(), BlockPopStatus::PoppedAndNonEmpty(Jamo::Vowel(JamoVowelSingular::A)));
    /// assert_eq!(composer.pop(), BlockPopStatus::PoppedAndEmpty(Jamo::Consonant(JamoConsonantSingular::Giyeok)));
    /// assert_eq!(composer.pop(), BlockPopStatus::None);
    /// ```
    pub fn pop(&mut self) -> BlockPopStatus {
        if let Some(c) = self.final_second.take() {
            self.state = BlockCompositionState::ExpectingCompositeFinal;
            BlockPopStatus::PoppedAndNonEmpty(c)
        } else if let Some(c) = self.final_first.take() {
            self.state = match self.vowel_second {
                Some(_) => BlockCompositionState::ExpectingFinal,
                None => BlockCompositionState::ExpectingCompositeVowelOrFinal,
            };
            BlockPopStatus::PoppedAndNonEmpty(c)
        } else if let Some(c) = self.vowel_second.take() {
            self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
            BlockPopStatus::PoppedAndNonEmpty(c)
        } else if let Some(c) = self.vowel_first.take() {
            self.state = match self.initial_second {
                Some(_) => BlockCompositionState::ExpectingVowel,
                None => BlockCompositionState::ExpectingDoubleInitialOrVowel,
            };
            BlockPopStatus::PoppedAndNonEmpty(c)
        } else if let Some(c) = self.initial_second.take() {
            self.state = BlockCompositionState::ExpectingVowel;
            BlockPopStatus::PoppedAndNonEmpty(c)
        } else if let Some(c) = self.initial_first.take() {
            self.state = BlockCompositionState::ExpectingInitial;
            BlockPopStatus::PoppedAndEmpty(c)
        } else {
            self.state = BlockCompositionState::ExpectingInitial;
            BlockPopStatus::None
        }
    }

    pub(crate) fn pop_end_consonant(&mut self) -> Option<Jamo> {
        if let Some(c) = self.final_second.take() {
            Some(c)
        } else if let Some(c) = self.final_first.take() {
            Some(c)
        } else {
            None
        }
    }

    fn try_push_initial(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Consonant(_) => {
                self.initial_first = Some(letter.clone());
                self.state = BlockCompositionState::ExpectingDoubleInitialOrVowel;
                BlockPushResult::Success
            }
            Jamo::CompositeConsonant(c) => {
                if c.is_valid_initial() {
                    self.initial_first = Some(letter.clone());
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
                match &self.initial_first {
                    Some(Jamo::Consonant(i1)) => {
                        if i1.combine_for_initial(c).is_some() {
                            self.initial_second = Some(letter.clone());
                            self.state = BlockCompositionState::ExpectingVowel;
                            BlockPushResult::Success
                        } else {
                            BlockPushResult::InvalidHangul
                        }
                    },
                    _ => BlockPushResult::InvalidHangul,
                }
            }
            Jamo::Vowel(_) => {
                self.vowel_first = Some(letter.clone());
                self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
                BlockPushResult::Success
            }
            Jamo::CompositeVowel(c) => match c.decompose() {
                (v1, v2) => {
                    self.vowel_first = Some(v1);
                    self.vowel_second = Some(v2);
                    self.state = BlockCompositionState::ExpectingFinal;
                    BlockPushResult::Success
                }
            }
            Jamo::CompositeConsonant(_) => BlockPushResult::InvalidHangul,
        }
    }

    fn try_push_vowel(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Vowel(_) => {
                self.vowel_first = Some(letter.clone());
                self.state = BlockCompositionState::ExpectingCompositeVowelOrFinal;
                BlockPushResult::Success
            }
            Jamo::CompositeVowel(c) => match c.decompose() {
                (v1, v2) => {
                    self.vowel_first = Some(v1);
                    self.vowel_second = Some(v2);
                    self.state = BlockCompositionState::ExpectingFinal;
                    BlockPushResult::Success
                }
            },
            _ => BlockPushResult::InvalidHangul,
        }
    }

    fn try_push_composite_vowel_or_final(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Vowel(c) => match &self.vowel_first {
                Some(Jamo::Vowel(v1)) => {
                    if v1.combine(c).is_some() {
                        self.vowel_second = Some(letter.clone());
                        self.state = BlockCompositionState::ExpectingFinal;
                        BlockPushResult::Success
                    } else {
                        BlockPushResult::InvalidHangul
                    }
                },
                _ => BlockPushResult::InvalidHangul,
            },
            Jamo::Consonant(_) => {
                self.final_first = Some(letter.clone());
                self.state = BlockCompositionState::ExpectingCompositeFinal;
                BlockPushResult::Success
            }
            Jamo::CompositeConsonant(c) => {
                if c.is_valid_final() {
                    match c.decompose() {
                        (f1, f2) => {
                            self.final_first = Some(f1);
                            self.final_second = Some(f2);
                            self.state = BlockCompositionState::ExpectingNextBlock;
                            BlockPushResult::Success
                        }
                    }
                }
                else if c.is_valid_initial() {
                    BlockPushResult::StartNewBlockNoPop
                }
                else {
                    BlockPushResult::InvalidHangul
                }
            },
            _ => BlockPushResult::InvalidHangul,
        }
    }

    fn try_push_final(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Consonant(_) => {
                self.final_first = Some(letter.clone());
                self.state = BlockCompositionState::ExpectingCompositeFinal;
                BlockPushResult::Success
            }
            Jamo::CompositeConsonant(c) => {
                if c.is_valid_final() {
                    match c.decompose() {
                        (f1, f2) => {
                            self.final_first = Some(f1);
                            self.final_second = Some(f2);
                            self.state = BlockCompositionState::ExpectingNextBlock;
                            BlockPushResult::Success
                        }
                    }
                }
                else if c.is_valid_initial() {
                    BlockPushResult::StartNewBlockNoPop
                }
                else {
                    BlockPushResult::InvalidHangul
                }
            },
            _ => BlockPushResult::InvalidHangul,
        }
    }

    fn try_push_composite_final(&mut self, letter: &Jamo) -> BlockPushResult {
        match letter {
            Jamo::Consonant(c) => {
                match &self.final_first {
                    Some(Jamo::Consonant(f1)) => {
                        if f1.combine_for_final(c).is_some() {
                            self.final_second = Some(letter.clone());
                            self.state = BlockCompositionState::ExpectingNextBlock;
                            BlockPushResult::Success
                        } else {
                            BlockPushResult::StartNewBlockNoPop
                        }
                    },
                    _ => BlockPushResult::InvalidHangul,
                }
            }
            Jamo::CompositeConsonant(c) => {
                if c.is_valid_initial() {
                    BlockPushResult::StartNewBlockNoPop
                }
                else {
                    BlockPushResult::InvalidHangul
                }
            },
            _ => BlockPushResult::PopAndStartNewBlock,
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
    /// use hangul::jamo::{Jamo, JamoConsonantSingular, JamoVowelSingular};
    ///
    /// let mut composer = BlockComposer::new();
    ///
    /// composer.push(&Jamo::from_compatibility_jamo('ㄱ').unwrap());
    ///
    /// // Attempt to complete incomplete block
    /// assert_eq!(
    ///     composer.try_as_complete_block(),
    ///     Ok(BlockCompletionStatus::Incomplete(Jamo::Consonant(JamoConsonantSingular::Giyeok)))
    /// );
    ///
    /// composer.push(&Jamo::from_compatibility_jamo('ㅏ').unwrap());
    ///
    /// // Get the complete block now that a vowel has been added
    /// assert_eq!(
    ///    composer.try_as_complete_block(),
    ///    Ok(BlockCompletionStatus::Complete(HangulBlock {
    ///        initial: Jamo::Consonant(JamoConsonantSingular::Giyeok),
    ///        vowel: Jamo::Vowel(JamoVowelSingular::A),
    ///        final_optional: None,
    ///    }))
    /// );
    /// ```
    pub fn try_as_complete_block(&self) -> Result<BlockCompletionStatus, String> {
        let initial_optional = match (&self.initial_first, &self.initial_second) {
            (Some(Jamo::Consonant(i1)), Some(Jamo::Consonant(i2))) => Some(
                Jamo::CompositeConsonant(i1.combine_for_initial(&i2).ok_or_else(|| {
                    format!("Invalid composite initial consonant: {:?}{:?}", i1, i2)
                })?),
            ),
            (Some(i1), None) => Some(i1.clone()),
            _ => None,
        };
        let vowel_optional = match (&self.vowel_first, &self.vowel_second) {
            (Some(Jamo::Vowel(v1)), Some(Jamo::Vowel(v2))) => Some(
                Jamo::CompositeVowel(v1.combine(&v2).ok_or_else(|| {
                    format!("Invalid composite vowel: {:?}{:?}", v1, v2)
                })?),
            ),
            (Some(v1), None) => Some(v1.clone()),
            _ => None,
        };
        let final_optional = match (&self.final_first, &self.final_second) {
            (Some(Jamo::Consonant(f1)), Some(Jamo::Consonant(f2))) => Some(
                Jamo::CompositeConsonant(f1.combine_for_final(&f2).ok_or_else(|| {
                    format!("Invalid composite final consonant: {:?}{:?}", f1, f2)
                })?),
            ),
            (Some(f1), None) => Some(f1.clone()),
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

    /// Returns the composed Hangul syllable character as an `Option<char>`
    /// wrapped in a `Result`. If the block is complete, it returns the composed
    /// character. If the block is incomplete, it returns the Jamo currently in
    /// the block (in modern Unicode form, not compatibility form). If the block is empty,
    /// it returns `None`.
    pub fn block_as_string(&self) -> Result<Option<char>, String> {
        match self.try_as_complete_block()? {
            BlockCompletionStatus::Complete(block) => block
                .to_char()
                .map(Some)
                .map_err(|e| format!("Error converting block to char: U+{:04X}", e)),
            BlockCompletionStatus::Incomplete(c) => Ok(c.char_modern(match c {
                Jamo::Consonant(_) | Jamo::CompositeConsonant(_) => JamoPosition::Initial,
                Jamo::Vowel(_) | Jamo::CompositeVowel(_) => JamoPosition::Vowel,
            })),
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
            initial: Jamo::from_compatibility_jamo('ㄱ').unwrap(),
            vowel: Jamo::from_compatibility_jamo('ㅏ').unwrap(),
            final_optional: Some(Jamo::from_compatibility_jamo('ㄴ').unwrap()),
        };
        let result = block.to_char();
        assert_eq!(result, Ok('간'));

        let block_no_final = HangulBlock {
            initial: Jamo::from_compatibility_jamo('ㅂ').unwrap(),
            vowel: Jamo::from_compatibility_jamo('ㅗ').unwrap(),
            final_optional: None,
        };
        let result_no_final = block_no_final.to_char();
        assert_eq!(result_no_final, Ok('보'));
    }

    #[test]
    fn test_hangul_blocks_vec_to_string() {
        let blocks = vec![
            HangulBlock {
                initial: Jamo::from_compatibility_jamo('ㅇ').unwrap(),
                vowel: Jamo::from_compatibility_jamo('ㅏ').unwrap(),
                final_optional: Some(Jamo::from_compatibility_jamo('ㄴ').unwrap()),
            },
            HangulBlock {
                initial: Jamo::from_compatibility_jamo('ㄴ').unwrap(),
                vowel: Jamo::from_compatibility_jamo('ㅕ').unwrap(),
                final_optional: Some(Jamo::from_compatibility_jamo('ㅇ').unwrap()),
            },
            HangulBlock {
                initial: Jamo::from_compatibility_jamo('ㅎ').unwrap(),
                vowel: Jamo::from_compatibility_jamo('ㅏ').unwrap(),
                final_optional: None,
            },
            HangulBlock {
                initial: Jamo::from_compatibility_jamo('ㅅ').unwrap(),
                vowel: Jamo::from_compatibility_jamo('ㅔ').unwrap(),
                final_optional: None,
            },
            HangulBlock {
                initial: Jamo::from_compatibility_jamo('ㅇ').unwrap(),
                vowel: Jamo::from_compatibility_jamo('ㅛ').unwrap(),
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
                input: vec![Jamo::from_compatibility_jamo('ㄱ').unwrap()],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingDoubleInitialOrVowel,
            },
            BlockComposerPushTestCase {
                input: vec![Jamo::from_compatibility_jamo('ㄱ').unwrap(), Jamo::from_compatibility_jamo('ㄱ').unwrap()],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingVowel,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::from_compatibility_jamo('ㄱ').unwrap(),
                    Jamo::from_compatibility_jamo('ㄱ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅜ').unwrap(),
                ],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingCompositeVowelOrFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::from_compatibility_jamo('ㄱ').unwrap(),
                    Jamo::from_compatibility_jamo('ㄱ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅜ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅓ').unwrap(),
                ],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::from_compatibility_jamo('ㄱ').unwrap(),
                    Jamo::from_compatibility_jamo('ㄱ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅜ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅓ').unwrap(),
                    Jamo::from_compatibility_jamo('ㄹ').unwrap(),
                ],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingCompositeFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::from_compatibility_jamo('ㄱ').unwrap(),
                    Jamo::from_compatibility_jamo('ㄱ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅜ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅓ').unwrap(),
                    Jamo::from_compatibility_jamo('ㄹ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅎ').unwrap(),
                ],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::from_compatibility_jamo('ㄱ').unwrap(),
                    Jamo::from_compatibility_jamo('ㄱ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅜ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅓ').unwrap(),
                    Jamo::from_compatibility_jamo('ㄹ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅎ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅏ').unwrap(),
                ],
                expected_final_word_state: BlockPushResult::PopAndStartNewBlock,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::from_compatibility_jamo('ㅃ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅣ').unwrap(),
                    Jamo::from_compatibility_jamo('ㄳ').unwrap(),
                ],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
            BlockComposerPushTestCase {
                input: vec![Jamo::from_compatibility_jamo('ㅈ').unwrap(), Jamo::from_compatibility_jamo('ㅚ').unwrap()],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::from_compatibility_jamo('ㅉ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅢ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅃ').unwrap(),
                ],
                expected_final_word_state: BlockPushResult::StartNewBlockNoPop,
                expected_final_block_state: BlockCompositionState::ExpectingFinal,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::from_compatibility_jamo('ㅇ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅣ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅅ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅅ').unwrap(),
                ],
                expected_final_word_state: BlockPushResult::Success,
                expected_final_block_state: BlockCompositionState::ExpectingNextBlock,
            },
            BlockComposerPushTestCase {
                input: vec![
                    Jamo::from_compatibility_jamo('ㅇ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅣ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅅ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅅ').unwrap(),
                    Jamo::from_compatibility_jamo('ㅅ').unwrap(),
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
                input: vec![Jamo::from_compatibility_jamo('ㄱ').unwrap(), Jamo::from_compatibility_jamo('ㄹ').unwrap()],
                expected_final_word_state: BlockPushResult::InvalidHangul,
                expected_final_block_state: BlockCompositionState::ExpectingDoubleInitialOrVowel,
            },
            BlockComposerPushTestCase {
                input: vec![Jamo::from_compatibility_jamo('ㄱ').unwrap(), Jamo::from_compatibility_jamo('ㅏ').unwrap(), Jamo::from_compatibility_jamo('ㅏ').unwrap()],
                expected_final_word_state: BlockPushResult::InvalidHangul,
                expected_final_block_state: BlockCompositionState::ExpectingCompositeVowelOrFinal,
            },
        ];
        run_test_cases(test_cases);
    }

    #[derive(Debug)]
    struct BlockE2ETestCase((char, char, char, char));

    fn run_e2e_test_cases(case: BlockE2ETestCase) {
        let mut composer = BlockComposer::new();
        assert_eq!(composer.push(&Jamo::from_compatibility_jamo(case.0 .0).unwrap()), BlockPushResult::Success, "Failed at initial consonant for case {:?}", case);
        assert_eq!(composer.push(&Jamo::from_compatibility_jamo(case.0 .1).unwrap()), BlockPushResult::Success, "Failed at vowel for case {:?}", case);
        if case.0 .2 != '\0' {
            assert_eq!(composer.push(&Jamo::from_compatibility_jamo(case.0 .2).unwrap()), BlockPushResult::Success, "Failed at final consonant for case {:?}", case);
        }

        let block_char = composer.block_as_string().unwrap();
        assert_eq!(block_char, Some(case.0 .3), "Final composed character did not match expected for case {:?}", case);
    }

    #[test]
    fn test_valid_blocks_e2e() {
        let case_tuples: Vec<(char, char, char, char)> = vec![
            // no final consonant
            ('ㅂ', 'ㅛ', '\0', '뵤'),
            ('ㅈ', 'ㅕ', '\0', '져'),
            ('ㄷ', 'ㅑ', '\0', '댜'),
            ('ㄱ', 'ㅐ', '\0', '개'),
            ('ㅅ', 'ㅔ', '\0', '세'),
            ('ㅁ', 'ㅗ', '\0', '모'),
            ('ㄴ', 'ㅓ', '\0', '너'),
            ('ㅇ', 'ㅏ', '\0', '아'),
            ('ㅎ', 'ㅣ', '\0', '히'),
            ('ㅋ', 'ㅠ', '\0', '큐'),
            ('ㅌ', 'ㅜ', '\0', '투'),
            ('ㅊ', 'ㅡ', '\0', '츠'),
            ('ㄹ', 'ㅒ', '\0', '럐'),
            ('ㅍ', 'ㅖ', '\0', '폐'),
            ('ㅃ', 'ㅛ', '\0', '뾰'),
            ('ㅉ', 'ㅕ', '\0', '쪄'),
            ('ㄸ', 'ㅑ', '\0', '땨'),
            ('ㄲ', 'ㅐ', '\0', '깨'),
            ('ㅆ', 'ㅔ', '\0', '쎄'),
            ('ㅂ', 'ㅘ', '\0', '봐'),
            ('ㅈ', 'ㅙ', '\0', '좨'),
            ('ㄷ', 'ㅚ', '\0', '되'),
            ('ㄱ', 'ㅝ', '\0', '궈'),
            ('ㅅ', 'ㅞ', '\0', '쉐'),
            ('ㅁ', 'ㅟ', '\0', '뮈'),
            ('ㄴ', 'ㅢ', '\0', '늬'),
            
            // with final consonant
            ('ㅂ', 'ㅛ', 'ㅆ', '뵸'),
            ('ㅈ', 'ㅕ', 'ㄲ', '젺'),
            ('ㄷ', 'ㅑ', 'ㄳ', '댟'),
            ('ㄱ', 'ㅐ', 'ㄵ', '갡'),
            ('ㅅ', 'ㅔ', 'ㄶ', '섾'),
            ('ㅁ', 'ㅗ', 'ㄺ', '몱'),
            ('ㄴ', 'ㅓ', 'ㄻ', '넒'),
            ('ㅇ', 'ㅏ', 'ㄼ', '앏'),
            ('ㅎ', 'ㅣ', 'ㄽ', '힔'),
            ('ㅋ', 'ㅠ', 'ㄾ', '큝'),
            ('ㅌ', 'ㅜ', 'ㄿ', '툺'),
            ('ㅊ', 'ㅡ', 'ㅀ', '츯'),
            ('ㄹ', 'ㅒ', 'ㅄ', '럢'),
            ('ㅍ', 'ㅖ', 'ㅂ', '폡'),
            ('ㅃ', 'ㅛ', 'ㅈ', '뿆'),
            ('ㅉ', 'ㅕ', 'ㄷ', '쪋'),
            ('ㄸ', 'ㅑ', 'ㄱ', '땩'),
            ('ㄲ', 'ㅐ', 'ㅅ', '깻'),
            ('ㅆ', 'ㅔ', 'ㅁ', '쎔'),
            ('ㅂ', 'ㅘ', 'ㄴ', '봔'),
            ('ㅈ', 'ㅙ', 'ㅇ', '좽'),
            ('ㄷ', 'ㅚ', 'ㄹ', '될'),
            ('ㄱ', 'ㅝ', 'ㅋ', '궠'),
            ('ㅅ', 'ㅞ', 'ㅌ', '쉩'),
            ('ㅁ', 'ㅟ', 'ㅊ', '뮟'),
            ('ㄴ', 'ㅢ', 'ㅍ', '닆'),
        ];

        for tuple in case_tuples {
            run_e2e_test_cases(BlockE2ETestCase(tuple));
        }
    }
}
