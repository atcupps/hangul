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

enum BlockCompositionState {
    /// nothing, waiting for first consonant
    ExpectingInitial,

    /// ex. ㄷ -> ㄸ or 다
    ExpectingDoubleInitialOrVowel(char),

    /// ex. ㄸ -> 따
    ExpectingVowel(char),

    /// ex. 두 -> 둬 or 둔
    ExpectingDoubleVowelOrFinal(char, char),

    /// ex. 둬 -> 뒁
    ExpectingFinal(char, char),

    /// ex. 달 -> 닳
    ExpectingCompositeFinal(char, char, char),
}

enum WordCompositionState {
    Composable,
    Terminated(char),
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
            BlockCompositionState::ExpectingInitial => {
                self.cur_block = BlockCompositionState::ExpectingDoubleInitialOrVowel(c);
                WordCompositionState::Composable
            }
            BlockCompositionState::ExpectingDoubleInitialOrVowel(initial) => {
                // If there is an initial consonant, try to make a double consonant.
                // If not possible, terminate the current composition.
                if let Some(double) = consonant_doubles(initial, c) {
                    self.cur_block = BlockCompositionState::ExpectingVowel(double);
                    WordCompositionState::Composable
                } else {
                    WordCompositionState::Terminated(c)
                }
            }
            BlockCompositionState::ExpectingVowel(_) => {
                // Cannot have two consonants in a row; terminate.
                WordCompositionState::Terminated(c)
            }
            BlockCompositionState::ExpectingDoubleVowelOrFinal(i, v) => {
                // Final consonant detected.
                self.cur_block = BlockCompositionState::ExpectingFinal(i, v);
                WordCompositionState::Composable
            }
            BlockCompositionState::ExpectingFinal(i, v) => {
                self.cur_block = BlockCompositionState::ExpectingCompositeFinal(i, v, c);
                WordCompositionState::Composable
            }
            BlockCompositionState::ExpectingCompositeFinal(i, v, f) => {
                if let Some(composite) = composite_final(f, c) {
                    self.cur_block =
                        BlockCompositionState::ExpectingCompositeFinal(i, v, composite);
                    WordCompositionState::Composable
                } else {
                    WordCompositionState::Terminated(c)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::determine_hangul;
    use super::{HangulLetter, Letter};

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
}
