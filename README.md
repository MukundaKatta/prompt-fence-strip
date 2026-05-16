# prompt-fence-strip

[![crates.io](https://img.shields.io/crates/v/prompt-fence-strip.svg)](https://crates.io/crates/prompt-fence-strip)

Strip ```code fences```, leading prose, and trailing chatter from LLM
output so the structured payload survives.

```rust
use prompt_fence_strip::strip_fences;
let raw = "Sure!\n```json\n{\"a\":1}\n```\nLet me know!";
assert_eq!(strip_fences(raw), "{\"a\":1}");
```

Returns trimmed input when no fence is present. Zero deps.
MIT or Apache-2.0.
