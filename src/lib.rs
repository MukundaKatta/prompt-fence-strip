//! # prompt-fence-strip
//!
//! Strip ```code fences```, leading prose ("Sure, here you go:"), and
//! trailing chatter ("Let me know if you need anything else!") from
//! LLM output so the structured payload inside survives.
//!
//! ## Example
//!
//! ```
//! use prompt_fence_strip::strip_fences;
//! let raw = "Sure!\n```json\n{\"a\":1}\n```\nLet me know!";
//! assert_eq!(strip_fences(raw), "{\"a\":1}");
//! ```

#![deny(missing_docs)]

/// Strip ```code fences``` and surrounding prose. Returns the *inner*
/// content of the first fenced block. If no fenced block is found, the
/// input is returned trimmed of surrounding whitespace.
pub fn strip_fences(s: &str) -> String {
    if let Some(body) = first_fenced_body(s) {
        return body.trim_matches('\n').to_string();
    }
    s.trim().to_string()
}

/// Strip without copying when no change is needed. Returns a borrowed
/// slice into `s` when possible, else an owned `String`.
pub fn strip_fences_cow(s: &str) -> std::borrow::Cow<'_, str> {
    if let Some(body) = first_fenced_body(s) {
        std::borrow::Cow::Owned(body.trim_matches('\n').to_string())
    } else {
        let t = s.trim();
        if t.len() == s.len() {
            std::borrow::Cow::Borrowed(s)
        } else {
            std::borrow::Cow::Owned(t.to_string())
        }
    }
}

fn first_fenced_body(s: &str) -> Option<&str> {
    let bytes = s.as_bytes();
    // Find the first ``` (3 backticks).
    let mut i = 0;
    while i + 2 < bytes.len() {
        if &bytes[i..i + 3] == b"```" {
            // Skip any "json", "rust", etc. tag plus the line break.
            let mut start = i + 3;
            while start < bytes.len() && bytes[start] != b'\n' {
                start += 1;
            }
            if start >= bytes.len() {
                return None;
            }
            start += 1; // skip the newline
            // Find the closing ``` (must be at start of a line or end-of-input).
            let mut j = start;
            while j + 3 <= bytes.len() {
                if &bytes[j..j + 3] == b"```" {
                    // Don't accept ``` mid-line — require LF before.
                    let prev = j.checked_sub(1).map(|k| bytes[k]).unwrap_or(b'\n');
                    if prev == b'\n' {
                        return Some(&s[start..j]);
                    }
                }
                j += 1;
            }
            return None;
        }
        i += 1;
    }
    None
}
