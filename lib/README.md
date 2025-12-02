## hangul-cd

*(hangul compose and decompose)*

Rust helpers for composing and decomposing Hangul syllable blocks and words. The crate in `lib/` exposes small, focused modules for combining jamo into syllables, grouping syllables into words, and mixing Hangul with arbitrary text while enforcing valid Unicode Hangul composition rules.

### Modules

hangul-cd focuses heavily on composition through a modular wrapper approach. There are four modules, each providing an API wrapper layer over the previous one:
- `jamo` - The Jamo layer provides utilities for working directly with individual Hangul Jamo; Jamo are stored as enum values, allowing for instantiation from and conversion to either Modern or Compatibility Unicode codepoints, as well as helpers for classification of Jamo.
- `block` - The Block layer provides a `HangulBlock` struct for composing syllable blocks from individual Jamo, as well as a `BlockComposer` which allows callers to use a simple stack-like push-pop interface to add to or remove from a given block, and functions to convert blocks to Unicode codepoints, as well as take completed blocks and decompose them into individual Modern or Compatibility Jamo codepoints.
- `word` - The Word layer wraps the Block layer by keeping track of a list of Hangul blocks and extends the push-pop mechanism from the Block layer, allowing callers to create full Hangul words composed of multiple syllable blocks simply by pushing Jamo repeatedly and print to Unicode codepoints.
- `string` - The String layer allows for mixing of Hangul and non-Hangul text and continues to make use of the push-pop mechanism from previous layers.

#### jamo

Work with individual Hangul letters, normalize compatibility codepoints, and compose or decompose composite Jamo.
```rust
use hangul::jamo::{
    Character,
    Jamo,
    JamoConsonantComposite,
    JamoConsonantSingular,
    JamoPosition,
    JamoVowelSingular,
    modernized_jamo_initial,
};

// Convert compatibility to modern codepoints or classify any char
assert_eq!(modernized_jamo_initial('ㄱ'), '\u{1100}');
assert!(matches!(Character::from_char('ㅙ').unwrap(), Character::Hangul(_)));

// Build and inspect Jamo values directly; works with either
// compatibility or modern Jamo Unicode codepoints
let jamo = Jamo::from_compatibility_jamo('ㅗ').unwrap();
assert_eq!(jamo.char_modern(JamoPosition::Vowel), Some('ᅩ'));

// Compose composite consonants programmatically
let double = JamoConsonantSingular::Giyeok.combine_for_initial(&JamoConsonantSingular::Giyeok);
assert_eq!(double, Some(JamoConsonantComposite::SsangGiyeok));
```

#### block

Compose and decompose single syllable blocks.
```rust
use hangul::block::{
    BlockComposer,
    BlockCompletionStatus,
    HangulBlock,
    HangulBlockDecompositionOptions,
};
use hangul::jamo::{Jamo, JamoConsonantSingular, JamoUnicodeType, JamoVowelSingular};

// Use `BlockComposer` to easily work with `HangulBlock`s
let mut composer = BlockComposer::new();

// Push directly using Jamo or with chars
composer.push(&Jamo::Consonant(JamoConsonantSingular::Giyeok));
composer.push(&Jamo::Vowel(JamoVowelSingular::A));
composer.push_char('ㅇ');

// Complete `BlockComposer` blocks as `HangulBlock` structs
let block = match composer.try_as_complete_block().unwrap() {
    BlockCompletionStatus::Complete(block) => block,
    _ => unreachable!(),
};

// Convert `HangulBlock`s to syllables
assert_eq!(block.to_char().unwrap(), '강');

// Decompose `HangulBlock`s into their constituent characters
let opts = HangulBlockDecompositionOptions {
    decompose_composites: true,
    jamo_era: JamoUnicodeType::Compatibility,
};
assert_eq!(block.decomposed_vec(&opts).unwrap(), vec!['ㄱ', 'ㅏ', 'ㅇ']);
```

#### word

Compose multiple syllables into a word with push/pop semantics.
```rust
use hangul::word::{HangulWordComposer, WordPushResult};

// Use `HangulWordComposer` to push and pop Jamo to words
let mut word = HangulWordComposer::new();
for c in "ㅇㅏㄴㄴㅕㅇ".chars() {
    assert_eq!(word.push_char(c).unwrap(), WordPushResult::Continue);
}
assert_eq!(word.as_string().unwrap(), "안녕".to_string());

// Backspace-like pops remove Jamo from a word one at a time
word.pop().unwrap();
assert_eq!(word.as_string().unwrap(), "안녀".to_string());
```

#### string

Mix Hangul words with arbitrary text.
```rust
use hangul::string::StringComposer;

// Use a `StringComposer` to push and pop both Hangul and non-Hangul chars
let mut s = StringComposer::new();
for c in "ㅎㅏㄴㄱㅡㄹ rocks".chars() {
    s.push_char(c).unwrap();
}
assert_eq!(s.as_string().unwrap(), "한글 rocks".to_string());

s.pop().unwrap(); // remove 's'
s.push_char('!').unwrap();
assert_eq!(s.as_string().unwrap(), "한글 rock!".to_string());
```

### Quick start

Add the crate to your project (for a local path):
```toml
[dependencies]
hangul = { path = "lib" }
```

Compose a single block or whole words:
```rust
use hangul::block::HangulBlock;
use hangul::string::StringComposer;
use hangul::jamo::Jamo;

// Manual block construction
let block = HangulBlock {
    initial: 'ㄱ',
    vowel: 'ㅏ',
    final_optional: Some('ㅇ'),
};
assert_eq!(block.to_char().unwrap(), '강');

// Stream jamo into a string
let mut composer = StringComposer::new();
for c in "ㅎㅏㄴㄱㅡㄹ ㅇㅏㄴㄴㅕㅇ!".chars() {
    composer.push_char(c).unwrap();
}
assert_eq!(composer.as_string().unwrap(), "한글 안녕!".to_string());

// Pop behaves like backspace
composer.pop().unwrap(); // removes '!'
assert_eq!(composer.as_string().unwrap(), "한글 안녕".to_string());
```

Work directly with jamo or compatibility jamo:
```rust
use hangul::jamo::{create_composite_initial, modernize_jamo_initial, Character};

assert_eq!(modernize_jamo_initial('\u{3131}'), '\u{1100}'); // ㄱ -> modern jamo
assert_eq!(create_composite_initial('ㄱ', 'ㄱ'), Some('ㄲ'));
assert!(matches!(Character::from_char('ㅘ'), Character::Hangul(_)));
```

### Testing

From `lib/`, run the library test suite:
```bash
cargo test
```
