use std::collections::HashSet;

use crate::chars::*;
/// Given an iterator over characters, composes them into Hangul syllables.
use crate::types::*;

const CONSONANTS: &str = "ㅂㅈㄷㄱㅅㅁㄴㅇㄹㅎㅋㅌㅊㅍ";
const VOWELS: &str = "ㅛㅕㅑㅐㅔㅒㅖㅗㅓㅏㅣㅠㅜㅡ";

/// Determines the type of Hangul letter for a given character.
/// Does NOT accept double consonants or composite vowels,
/// since the idea is that this function is called on each
/// individual character input. Also, does not work for
/// archaic jamo like ᅀ.
fn determine_hangul(c: char) -> Letter {
    return if CONSONANTS.contains(c) {
        Letter::Hangul(HangulLetter::Consonant(c))
    } else if VOWELS.contains(c) {
        Letter::Hangul(HangulLetter::Vowel(c))
    } else {
        Letter::NonHangul(c)
    };
}

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
    StartNewBlock(char),
    Invalid(char),
}

impl HangulWordComposer {
    pub fn new_word() -> Self {
        HangulWordComposer {
            prev_blocks: Vec::new(),
            cur_block: BlockCompositionState::ExpectingInitial,
        }
    }

    pub fn push(&mut self, letter: HangulLetter) -> WordCompositionState {
        match letter {
            HangulLetter::Consonant(c) => self.push_consonant(c),
            HangulLetter::Vowel(c) => self.push_vowel(c),
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
                    WordCompositionState::Invalid(c)
                }
            }

            // already has a double initial consonant and needs a vowel
            BlockCompositionState::ExpectingVowel(_) => WordCompositionState::Invalid(c),

            // Has a vowel, accepts a consonant as a final consonant
            BlockCompositionState::ExpectingCompositeVowelOrFinal(i, v) => {
                self.cur_block = BlockCompositionState::ExpectingFinal(i, v);
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
                    WordCompositionState::StartNewBlock(c)
                }
            }

            // Has a composite final consonant; next consonant starts a new block.
            BlockCompositionState::ExpectingNextBlock(_, _, _) => {
                WordCompositionState::StartNewBlock(c)
            }
        }
    }

    fn push_vowel(&mut self, c: char) -> WordCompositionState {
        match self.cur_block {
            // First letter must be a consonant
            BlockCompositionState::ExpectingInitial => WordCompositionState::Invalid(c),

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
                    WordCompositionState::Invalid(c)
                }
            }

            // already has composite vowel, cannot accept a third
            BlockCompositionState::ExpectingFinal(_, _) => WordCompositionState::Invalid(c),

            // has a final consonant; a vowel indicates that this consonant is part of a new block
            BlockCompositionState::ExpectingCompositeFinalOrNextBlock(_, _, _) => {
                WordCompositionState::StartNewBlock(c)
            }

            // Has a composite final consonant; a vowel starts a new block
            // with the end consonant as the initial of the new block.
            BlockCompositionState::ExpectingNextBlock(_, _, _) => {
                WordCompositionState::StartNewBlock(c)
            }
        }
    }
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
                result == Letter::Hangul(HangulLetter::Consonant(c)),
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
                result == Letter::Hangul(HangulLetter::Vowel(c)),
                "Failed on vowel: {}; got result: {:?}",
                c,
                result
            );
        }
    }

    #[test]
    fn determine_hangul_rejects_compound_letters() {
        let compound_letters = "ㄲㄸㅃㅆㅉㅘㅙㅚㅝㅞㅟㅢ";
        for c in compound_letters.chars() {
            let result = determine_hangul(c);
            assert!(
                result == Letter::NonHangul(c),
                "Failed on compound letter: {}; got result: {:?}",
                c,
                result
            );
        }
    }

    struct HangulWordComposerTestCase {
        input: Vec<HangulLetter>,
        expected_final_word_state: WordCompositionState,
        expected_final_block_state: BlockCompositionState,
        expected_prev_blocks: Vec<HangulBlock>,
    }

    fn run_test_cases(cases: Vec<HangulWordComposerTestCase>) {
        for case in cases {
            let mut composer = HangulWordComposer::new_word();
            let mut final_word_state = WordCompositionState::Composable;
            for letter in case.input {
                final_word_state = composer.push(letter);
            }
            assert_eq!(
                final_word_state, case.expected_final_word_state,
                "Final WORD state did not match expected."
            );
            assert_eq!(
                composer.cur_block, case.expected_final_block_state,
                "Final BLOCK state did not match expected."
            );
            assert_eq!(
                composer.prev_blocks, case.expected_prev_blocks,
                "Previous blocks did not match expected."
            );
        }
    }

    #[test]
    fn single_block_composition_valid() {
        let test_cases: Vec<HangulWordComposerTestCase> =
            vec![
                HangulWordComposerTestCase {
                    input: vec![HangulLetter::Consonant('ㄱ')],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state:
                        BlockCompositionState::ExpectingDoubleInitialOrVowel('ㄱ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerTestCase {
                    input: vec![HangulLetter::Consonant('ㄱ'), HangulLetter::Consonant('ㄱ')],
                    expected_final_word_state: WordCompositionState::Composable,
                    expected_final_block_state: BlockCompositionState::ExpectingVowel('ㄲ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerTestCase {
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
                HangulWordComposerTestCase {
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
                HangulWordComposerTestCase {
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
                HangulWordComposerTestCase {
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
                HangulWordComposerTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅜ'),
                        HangulLetter::Vowel('ㅓ'),
                        HangulLetter::Consonant('ㄹ'),
                        HangulLetter::Consonant('ㅎ'),
                        HangulLetter::Vowel('ㅏ'),
                    ],
                    expected_final_word_state: WordCompositionState::StartNewBlock('ㅏ'),
                    expected_final_block_state: BlockCompositionState::ExpectingNextBlock(
                        'ㄲ', 'ㅝ', 'ㅀ',
                    ),
                    expected_prev_blocks: vec![],
                },
            ];

        run_test_cases(test_cases);
    }

    #[test]
    fn single_block_composition_invalid() {
        let test_cases: Vec<HangulWordComposerTestCase> =
            vec![
                HangulWordComposerTestCase {
                    input: vec![HangulLetter::Consonant('ㄱ'), HangulLetter::Consonant('ㄹ')],
                    expected_final_word_state: WordCompositionState::Invalid('ㄹ'),
                    expected_final_block_state:
                        BlockCompositionState::ExpectingDoubleInitialOrVowel('ㄱ'),
                    expected_prev_blocks: vec![],
                },
                HangulWordComposerTestCase {
                    input: vec![
                        HangulLetter::Consonant('ㄱ'),
                        HangulLetter::Vowel('ㅏ'),
                        HangulLetter::Vowel('ㅏ'),
                    ],
                    expected_final_word_state: WordCompositionState::Invalid('ㅏ'),
                    expected_final_block_state:
                        BlockCompositionState::ExpectingCompositeVowelOrFinal('ㄱ', 'ㅏ'),
                    expected_prev_blocks: vec![],
                },
            ];
        run_test_cases(test_cases);
    }
}
