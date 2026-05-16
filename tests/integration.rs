use prompt_fence_strip::{strip_fences, strip_fences_cow};

#[test]
fn strips_json_fence_with_prose() {
    let raw = "Sure, here you go:\n```json\n{\"a\":1}\n```\nLet me know!";
    assert_eq!(strip_fences(raw), "{\"a\":1}");
}

#[test]
fn strips_unmarked_fence() {
    let raw = "```\nhello\n```";
    assert_eq!(strip_fences(raw), "hello");
}

#[test]
fn trims_when_no_fence() {
    assert_eq!(strip_fences("  hello  \n"), "hello");
}

#[test]
fn cow_borrows_when_no_change() {
    let s = "no-change-here";
    let out = strip_fences_cow(s);
    assert!(matches!(out, std::borrow::Cow::Borrowed(_)));
    assert_eq!(out, "no-change-here");
}

#[test]
fn cow_owns_when_fence_present() {
    let raw = "```\nhi\n```";
    let out = strip_fences_cow(raw);
    assert!(matches!(out, std::borrow::Cow::Owned(_)));
    assert_eq!(out, "hi");
}

#[test]
fn handles_multiline_body() {
    let raw = "```\nline1\nline2\nline3\n```";
    assert_eq!(strip_fences(raw), "line1\nline2\nline3");
}

#[test]
fn unterminated_fence_returns_trimmed_input() {
    // No closing — we return the trimmed original to be safe.
    let raw = "```json\n{\"a\":1}";
    assert_eq!(strip_fences(raw), "```json\n{\"a\":1}");
}
