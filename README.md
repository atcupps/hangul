## hangul-cd

*(hangul compose and decompose)*

Rust helpers for composing and decomposing modern Hangul syllable blocks from jamo. The crate in `lib/` exposes small, focused modules for combining jamo into syllables, grouping syllables into words, and mixing Hangul with arbitrary text while enforcing valid Unicode Hangul composition rules.

### Modules

hangul-cd focuses heavily on composition through a modular wrapper approach. There are four modules, each providing an API wrapper layer over the previous one:
- `jamo` - The Jamo layer provides utilities for working directly with individual Hangul Jamo; Jamo are stored as enum values, allowing for instantiation from and conversion to either Modern or Compatibility Unicode codepoints, as well as helpers for classification of Jamo.
- `block` - The Block layer provides a `HangulBlock` struct for composing syllable blocks from individual Jamo, as well as a `BlockComposer` which allows callers to use a simple stack-like push-pop interface to add to or remove from a given block, and functions to convert blocks to Unicode codepoints, as well as take completed blocks and decompose them into individual Modern or Compatibility Jamo codepoints.
- `word` - The Word layer wraps the Block layer by keeping track of a list of Hangul blocks and extends the push-pop mechanism from the Block layer, allowing callers to create full Hangul words composed of multiple syllable blocks simply by pushing Jamo repeatedly and print to Unicode codepoints.
- `string` - The String layer allows for mixing of Hangul and non-Hangul text and continues to make use of the push-pop mechanism from previous layers.

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
