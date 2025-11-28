use std::fmt::Debug;

use crate::{block::*, jamo::*};

#[derive(Debug)]
pub struct HangulWordComposer {
    prev_blocks: Vec<HangulBlock>,
    cur_block: BlockComposer,
}

impl HangulWordComposer {
    pub fn new() -> Self {
        HangulWordComposer {
            prev_blocks: Vec::new(),
            cur_block: BlockComposer::new(),
        }
    }

    pub fn push_char(&mut self, c: char) -> PushResult {
        match determine_hangul(c) {
            Character::Hangul(hl) => self.push(&hl),
            Character::NonHangul(_) => PushResult::NonHangul,
        }
    }

    pub fn push(&mut self, letter: &Jamo) -> PushResult {
        self.cur_block.push(letter)
    }

    pub fn pop(&mut self) -> Result<Option<Jamo>, String> {
        match self.cur_block.pop() {
            BlockPopStatus::PoppedAndShouldContinue(l) => Ok(Some(l)),
            BlockPopStatus::PoppedAndShouldRemove(l) => {
                self.prev_block_to_cur()?;
                Ok(Some(l))
            }
            BlockPopStatus::None => {
                self.prev_block_to_cur()?;
                Ok(None)
            }
        }
    }

    fn prev_block_to_cur(&mut self) -> Result<(), String> {
        if let Some(last_block) = self.prev_blocks.pop() {
            self.cur_block = BlockComposer::from_composed_block(&last_block)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn pop_and_start_new_block(&mut self, letter: Jamo) -> Result<(), String> {
        match self.cur_block.pop_end_consonant() {
            Some(l) => {
                self.complete_current_block()?;
                self.cur_block.push(&l);
                match self.cur_block.push(&letter) {
                    PushResult::Success => Ok(()),
                    _ => Err(format!(
                        "Error starting new block with letter: {:?}",
                        letter
                    )),
                }
            }
            None => Err(format!(
                "Could not pop end consonant in function start_new_block with letter: {:?}",
                letter
            )),
        }
    }

    pub fn start_new_block(&mut self, letter: Jamo) -> Result<(), String> {
        self.complete_current_block()?;
        match self.cur_block.push(&letter) {
            PushResult::Success => Ok(()),
            _ => Err(format!(
                "Error starting new block with letter: {:?}",
                letter
            )),
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
        match self.cur_block.try_as_complete_block()? {
            BlockCompletionStatus::Complete(block) => {
                self.prev_blocks.push(block);
                self.cur_block = BlockComposer::new();
                Ok(())
            }
            BlockCompletionStatus::Incomplete(c) => Err(format!(
                "Cannot complete current block: incomplete block state, leftover char: {}",
                c
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_new_block_valid() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push(&Jamo::Consonant('ㄱ')), PushResult::Success);
        assert_eq!(composer.push(&Jamo::Vowel('ㅏ')), PushResult::Success);
        assert_eq!(composer.push(&Jamo::Consonant('ㄴ')), PushResult::Success,);
        assert_eq!(
            composer.push(&Jamo::Consonant('ㅇ')),
            PushResult::StartNewBlockNoPop,
        );
        assert_eq!(composer.start_new_block(Jamo::Consonant('ㅇ')), Ok(()));
        assert_eq!(
            composer.prev_blocks,
            vec![HangulBlock {
                initial: 'ㄱ',
                vowel: 'ㅏ',
                final_optional: Some('ㄴ'),
            }]
        );
        assert_eq!(composer.push(&Jamo::Vowel('ㅛ')), PushResult::Success);
        assert_eq!(
            composer.push(&Jamo::CompositeConsonant('ㅉ')),
            PushResult::StartNewBlockNoPop,
        );
        assert_eq!(
            composer.start_new_block(Jamo::CompositeConsonant('ㅉ')),
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
        let mut composer = HangulWordComposer::new();

        assert_eq!(
            composer.start_new_block(Jamo::Vowel('ㅏ')),
            Err("Cannot form block: missing initial consonant and vowel".to_string())
        );
        let _ = composer.push(&Jamo::Consonant('ㄱ'));
        assert_eq!(
            composer.start_new_block(Jamo::CompositeVowel('ㅘ')),
            Err(
                "Cannot complete current block: incomplete block state, leftover char: ㄱ"
                    .to_string()
            )
        );
    }

    #[test]
    fn push_char_valid() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㄱ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::Success,);
    }

    #[test]
    fn push_char_invalid_hangul() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㄱ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄹ'), PushResult::Success,);
        assert_eq!(composer.push_char('ㄽ'), PushResult::InvalidHangul,);
    }

    #[test]
    fn push_char_next_block() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㄱ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::Success,);
        assert_eq!(composer.push_char('ㅇ'), PushResult::StartNewBlockNoPop,);
    }

    #[test]
    fn push_char_non_hangul() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㄱ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('A'), PushResult::NonHangul,);
    }

    #[test]
    fn test_single_word_안녕하세요_as_string() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::StartNewBlockNoPop);
        assert_eq!(composer.start_new_block(Jamo::Consonant('ㄴ')), Ok(()));
        assert_eq!(composer.push_char('ㅕ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅎ'), PushResult::StartNewBlockNoPop);
        assert_eq!(composer.start_new_block(Jamo::Consonant('ㅎ')), Ok(()));
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅅ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅔ'), PushResult::PopAndStartNewBlock);
        assert_eq!(composer.pop_and_start_new_block(Jamo::Vowel('ㅔ')), Ok(()));
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅛ'), PushResult::PopAndStartNewBlock);
        assert_eq!(composer.pop_and_start_new_block(Jamo::Vowel('ㅛ')), Ok(()));

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "안녕하세요".to_string());
    }

    #[test]
    fn test_single_word_앖어요_as_string() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅓ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅂ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅅ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅇ'), PushResult::StartNewBlockNoPop);
        assert_eq!(composer.start_new_block(Jamo::Consonant('ㅇ')), Ok(()));
        assert_eq!(composer.push_char('ㅓ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅛ'), PushResult::PopAndStartNewBlock);
        assert_eq!(composer.pop_and_start_new_block(Jamo::Vowel('ㅛ')), Ok(()));

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "없어요".to_string());
    }

    #[test]
    fn test_incomplete_block_as_string() {
        let mut composer = HangulWordComposer::new();

        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "ㅇ".to_string());
    }

    #[test]
    fn test_deletions() {
        let mut composer = HangulWordComposer::new();
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::StartNewBlockNoPop);
        assert_eq!(composer.start_new_block(Jamo::Consonant('ㄴ')), Ok(()));
        assert_eq!(composer.push_char('ㅕ'), PushResult::Success);

        assert_eq!(composer.pop(), Ok(Some(Jamo::Vowel('ㅕ'))));
        assert_eq!(composer.pop(), Ok(Some(Jamo::Consonant('ㄴ'))));
        assert_eq!(composer.pop(), Ok(Some(Jamo::Consonant('ㄴ'))));
        assert_eq!(composer.pop(), Ok(Some(Jamo::Vowel('ㅏ'))));
        assert_eq!(composer.pop(), Ok(Some(Jamo::Consonant('ㅇ'))));
        assert_eq!(composer.pop(), Ok(None));
    }

    #[test]
    fn test_deletion_then_write_again() {
        let mut composer = HangulWordComposer::new();
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::Success);

        assert_eq!(composer.pop(), Ok(Some(Jamo::Consonant('ㄴ'))));
        assert_eq!(composer.pop(), Ok(Some(Jamo::Vowel('ㅏ'))));
        assert_eq!(composer.pop(), Ok(Some(Jamo::Consonant('ㅇ'))));

        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::Success);

        let result_string = composer.as_string().unwrap();
        assert_eq!(result_string, "안".to_string());
    }

    #[test]
    fn deletion_removes_empty_block() {
        let mut composer = HangulWordComposer::new();
        assert_eq!(composer.push_char('ㅇ'), PushResult::Success);
        assert_eq!(composer.push_char('ㅏ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::Success);
        assert_eq!(composer.push_char('ㄴ'), PushResult::StartNewBlockNoPop);
        assert_eq!(composer.start_new_block(Jamo::Consonant('ㄴ')), Ok(()));

        assert_eq!(composer.pop(), Ok(Some(Jamo::Consonant('ㄴ'))));
        // if current block is still empty, as_string should fail
        assert_eq!(composer.as_string().unwrap(), "안".to_string());
    }
}
