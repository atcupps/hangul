## hangul-cd

*(hangul compose and decompose)*

Rust helpers for composing and decomposing modern Hangul syllable blocks from jamo. The crate in `lib/` exposes small, focused modules for combining jamo into syllables, grouping syllables into words, and mixing Hangul with arbitrary text while enforcing valid Unicode Hangul composition rules.

### Modules
- `jamo` – Modern jamo constants, helpers to create/decompose composite jamo, and converters from compatibility jamo to the modern code points. Also provides the `Character` classifier and the `Jamo` enum used across the crate.
- `block` – `HangulBlock` (initial, vowel, optional final) plus a `BlockComposer` state machine that only accepts valid jamo sequences. Includes helpers to convert blocks into Unicode syllable chars and back.
- `word` – `HangulWordComposer` wraps a `BlockComposer` and a list of completed blocks, letting you stream jamo in, pop them out, and automatically start new syllable blocks when needed.
- `string` – `StringComposer` layers on top of `HangulWordComposer` so you can interleave Hangul input with non-Hangul text; invalid/compatibility jamo are normalized and non-Hangul is passed through.

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
