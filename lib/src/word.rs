use std::fmt::Debug;

use thiserror::Error;

use crate::{block::*, jamo::*};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum WordError {
    #[error("Block error: {0}")]
    BlockError(#[from] BlockError),

    #[error("Jamo error: {0}")]
    JamoError(#[from] JamoError),

    #[error("Could not start new block with character '{0}'; reason: {1:?}")]
    CouldNotStartNewBlock(char, BlockPushResult),

    #[error("Tried popping from empty word")]
    NothingToPop,

    #[error("Cannot complete current block; currently contains only one Jamo: {0:?}")]
    CannotCompleteCurrentBlock(Jamo),
}

/// A composer for a single Hangul word, made up of multiple syllable blocks.
/// The `HangulWordComposer` maintains a list of completed `HangulBlock`s and a
/// `BlockComposer` for the current syllable block being composed.
///
/// **API:**
/// ```rust
/// use hangul::word::{HangulWordComposer, WordPushResult};
/// use hangul::jamo::{Jamo, JamoConsonantSingular, JamoVowelSingular};
///
/// let mut composer = HangulWordComposer::new();
///
/// // Push characters to form Hangul syllables
/// assert_eq!(composer.push_char('ㅇ').unwrap(), WordPushResult::Continue);
/// assert_eq!(composer.push_char('ㅏ').unwrap(), WordPushResult::Continue);
/// assert_eq!(composer.push_char('ㄴ').unwrap(), WordPushResult::Continue);
/// assert_eq!(composer.push_char('ㄴ').unwrap(), WordPushResult::Continue);
/// assert_eq!(composer.push_char('ㅕ').unwrap(), WordPushResult::Continue);
/// assert_eq!(composer.push_char('ㅇ').unwrap(), WordPushResult::Continue);
///
/// // Get the composed string
/// let result = composer.as_string().unwrap();
/// assert_eq!(result, "안녕".to_string());
///
/// // Popping characters removes jamo in reverse order
/// assert_eq!(
///     composer.pop().unwrap(),
///     Some(Jamo::Consonant(JamoConsonantSingular::Ieung))
/// );
/// assert_eq!(
///     composer.pop().unwrap(),
///     Some(Jamo::Vowel(JamoVowelSingular::Yeo))
/// );
/// assert_eq!(
///     composer.pop().unwrap(),
///     Some(Jamo::Consonant(JamoConsonantSingular::Nieun))
/// );
/// assert_eq!(composer.as_string().unwrap(), "안".to_string());
/// ```
#[derive(Debug)]
pub struct HangulWordComposer {
    prev_blocks: Vec<HangulBlock>,
    cur_block: BlockComposer,
}

/// The result of attempting to push a character into the `HangulWordComposer`.
#[derive(Debug, PartialEq, Eq)]
pub enum WordPushResult {
    /// The character was successfully pushed and composition can continue.
    Continue,

    /// The character could not be pushed because it would result in an invalid
    /// Hangul syllable.
    InvalidHangul,

    /// The character was not pushed because it is not a Hangul character.
    NonHangul,
}

impl HangulWordComposer {
    /// Creates a new, empty `HangulWordComposer`.
    pub fn new() -> Self {
        HangulWordComposer {
            prev_blocks: Vec::new(),
            cur_block: BlockComposer::new(),
        }
    }

    /// Pushes a character into the `HangulWordComposer` if valid and returns a
    /// result indicating the outcome.
    ///
    /// If pushing would make a valid Hangul syllable, the new character is
    /// appended and `WordPushResult::Continue` is returned.
    ///
    /// If pushing the character would result in an invalid Hangul syllable,
    /// but the character is Hangul and can start a new syllable block, the current
    /// block is completed, a new block is started with the pushed character,
    /// and `WordPushResult::Continue` is returned.
    ///
    /// If the character is Hangul but cannot form a valid syllable in either
    /// the current or a new block, `WordPushResult::InvalidHangul` is returned.
    ///
    /// If the character is not Hangul, `WordPushResult::NonHangul` is returned.
    pub fn push_char(&mut self, c: char) -> Result<WordPushResult, WordError> {
        match Character::from_char(c)? {
            Character::Hangul(jamo) => self.push(&jamo),
            Character::NonHangul(_) => Ok(WordPushResult::NonHangul),
        }
    }

    /// Pushes a Jamo letter into the `HangulWordComposer`. Acts the same as
    /// `push_char`, but takes a `Jamo` instead of a `char`.
    /// Pushing appends to the current syllable block if that would make a
    /// valid Hangul syllable; otherwise, it completes the current block and
    /// creates a new block with the pushed character.
    pub fn push(&mut self, letter: &Jamo) -> Result<WordPushResult, WordError> {
        match self.cur_block.push(letter) {
            BlockPushResult::Success => Ok(WordPushResult::Continue),
            BlockPushResult::InvalidHangul => Ok(WordPushResult::InvalidHangul),
            BlockPushResult::NonHangul => Ok(WordPushResult::NonHangul),
            BlockPushResult::StartNewBlockNoPop => match self.start_new_block(letter.clone()) {
                Ok(_) => Ok(WordPushResult::Continue),
                Err(e) => Err(e),
            },
            BlockPushResult::PopAndStartNewBlock => {
                match self.pop_and_start_new_block(letter.clone()) {
                    Ok(_) => Ok(WordPushResult::Continue),
                    Err(e) => Err(e),
                }
            }
        }
    }

    /// Pops the last Jamo letter from the `HangulWordComposer`.
    /// If the current syllable block has letters, it will remove the last
    /// letter from it. If the current block is empty, it will set the last
    /// completed block as the currently-active block and remove one Jamo
    /// from it if possible.
    ///
    /// Returns `Ok(Some(Jamo))` if a letter was successfully removed,
    /// `Ok(None)` if there are no letters to remove, or `Err(String)` if an
    /// error occurred during the operation.
    pub fn pop(&mut self) -> Result<Option<Jamo>, WordError> {
        match self.cur_block.pop() {
            BlockPopStatus::PoppedAndNonEmpty(l) => Ok(Some(l)),
            BlockPopStatus::PoppedAndEmpty(l) => {
                self.prev_block_to_cur()?;
                Ok(Some(l))
            }
            BlockPopStatus::None => {
                self.prev_block_to_cur()?;
                Ok(None)
            }
        }
    }

    fn prev_block_to_cur(&mut self) -> Result<(), WordError> {
        if let Some(last_block) = self.prev_blocks.pop() {
            self.cur_block = BlockComposer::from_composed_block(&last_block)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn pop_and_start_new_block(&mut self, letter: Jamo) -> Result<(), WordError> {
        match self.cur_block.pop_end_consonant() {
            Some(l) => {
                self.complete_current_block()?;
                self.cur_block.push(&l);
                match self.cur_block.push(&letter) {
                    BlockPushResult::Success => Ok(()),
                    other => Err(WordError::CouldNotStartNewBlock(
                        letter.char_compatibility(),
                        other,
                    )),
                }
            }
            None => Err(WordError::NothingToPop),
        }
    }

    fn start_new_block(&mut self, letter: Jamo) -> Result<(), WordError> {
        self.complete_current_block()?;
        match self.cur_block.push(&letter) {
            BlockPushResult::Success => Ok(()),
            other => Err(WordError::CouldNotStartNewBlock(
                letter.char_compatibility(),
                other,
            )),
        }
    }

    /// Returns the composed string for the current Hangul word.
    /// This includes all completed syllable blocks and the current block,
    /// even if it is incomplete.
    pub fn as_string(&self) -> Result<String, WordError> {
        let mut result = hangul_blocks_vec_to_string(&self.prev_blocks)?;
        let cur_as_char = self.cur_block.block_as_string()?;
        if let Some(c) = cur_as_char {
            result.push(c);
        }
        Ok(result)
    }

    fn complete_current_block(&mut self) -> Result<(), WordError> {
        match self.cur_block.try_as_complete_block()? {
            BlockCompletionStatus::Complete(block) => {
                self.prev_blocks.push(block);
                self.cur_block = BlockComposer::new();
                Ok(())
            }
            BlockCompletionStatus::Incomplete(c) => Err(WordError::CannotCompleteCurrentBlock(c)),
            BlockCompletionStatus::Empty => {
                // Nothing to complete
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_new_block_valid() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㄱ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue),);
        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue),);
        assert_eq!(
            composer.prev_blocks,
            vec![HangulBlock {
                initial: Jamo::Consonant(JamoConsonantSingular::Giyeok),
                vowel: Jamo::Vowel(JamoVowelSingular::A),
                final_optional: Some(Jamo::Consonant(JamoConsonantSingular::Nieun)),
            }]
        );
        assert_eq!(composer.push_char('ㅛ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅉ'), Ok(WordPushResult::Continue),);
        assert_eq!(
            composer.prev_blocks,
            vec![
                HangulBlock {
                    initial: Jamo::Consonant(JamoConsonantSingular::Giyeok),
                    vowel: Jamo::Vowel(JamoVowelSingular::A),
                    final_optional: Some(Jamo::Consonant(JamoConsonantSingular::Nieun)),
                },
                HangulBlock {
                    initial: Jamo::Consonant(JamoConsonantSingular::Ieung),
                    vowel: Jamo::Vowel(JamoVowelSingular::Yo),
                    final_optional: None,
                }
            ]
        );
    }

    #[test]
    fn start_new_block_invalid() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(
            composer.start_new_block(Jamo::Vowel(JamoVowelSingular::A)),
            Err(WordError::CouldNotStartNewBlock(
                'ㅏ',
                BlockPushResult::InvalidHangul
            ))
        );
        let _ = composer.push_char('ㄱ');
        assert_eq!(
            composer.start_new_block(Jamo::CompositeVowel(JamoVowelComposite::Wae)),
            Err(WordError::CannotCompleteCurrentBlock(Jamo::Consonant(
                JamoConsonantSingular::Giyeok
            )))
        );
    }

    #[test]
    fn push_char_valid() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㄱ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue),);
    }

    #[test]
    fn push_char_invalid_hangul() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㄱ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄹ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄽ'), Ok(WordPushResult::InvalidHangul));
    }

    #[test]
    fn push_char_next_block() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㄱ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
    }

    #[test]
    fn push_char_non_hangul() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㄱ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('A'), Ok(WordPushResult::NonHangul));
    }

    #[test]
    fn test_single_word_안녕하세요_as_string() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅕ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅎ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅅ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅔ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅛ'), Ok(WordPushResult::Continue));

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "안녕하세요".to_string());
    }

    #[test]
    fn test_single_word_앖어요_as_string() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅓ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅂ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅅ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅓ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅛ'), Ok(WordPushResult::Continue));

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "없어요".to_string());
    }

    #[test]
    fn test_incomplete_block_as_string() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "ᄋ".to_string());
    }

    #[test]
    fn test_deletions() {
        let mut composer = HangulWordComposer::new();
        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅕ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.pop().unwrap().unwrap().char_compatibility(), 'ㅕ');
        assert_eq!(composer.pop().unwrap().unwrap().char_compatibility(), 'ㄴ');
        assert_eq!(composer.pop().unwrap().unwrap().char_compatibility(), 'ㄴ');
        assert_eq!(composer.pop().unwrap().unwrap().char_compatibility(), 'ㅏ');
        assert_eq!(composer.pop().unwrap().unwrap().char_compatibility(), 'ㅇ');
        assert_eq!(composer.pop(), Ok(None));
    }

    #[test]
    fn test_deletion_then_write_again() {
        let mut composer = HangulWordComposer::new();
        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue));

        assert_eq!(composer.pop().unwrap().unwrap().char_compatibility(), 'ㄴ');
        assert_eq!(composer.pop().unwrap().unwrap().char_compatibility(), 'ㅏ');
        assert_eq!(composer.pop().unwrap().unwrap().char_compatibility(), 'ㅇ');

        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue));

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "안".to_string());
    }

    #[test]
    fn deletion_removes_empty_block() {
        let mut composer = HangulWordComposer::new();
        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue));

        assert_eq!(composer.pop().unwrap().unwrap().char_compatibility(), 'ㄴ');
        // if current block is still empty, as_string should fail
        assert_eq!(composer.as_string().unwrap(), "안".to_string());
    }

    #[test]
    fn test_complete_current_block() {
        let mut composer = HangulWordComposer::new();
        assert_eq!(composer.push_char('ㅇ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㅏ'), Ok(WordPushResult::Continue));
        assert_eq!(composer.push_char('ㄴ'), Ok(WordPushResult::Continue));

        assert!(composer.complete_current_block().is_ok());

        assert_eq!(composer.prev_blocks.len(), 1);
        assert_eq!(composer.cur_block, BlockComposer::new());

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "안".to_string());
    }
}
