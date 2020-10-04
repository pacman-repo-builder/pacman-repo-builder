use super::split_str_once;

pub fn extract_pkgname_prefix(text: &str) -> (&str, &str) {
    split_str_once(text, |current_char, _| match current_char {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '@' | '.' | '_' | '+' | '-' => false,
        _ => true,
    })
}

#[test]
fn test_extract_pkgname_prefix_partial() {
    assert_eq!(extract_pkgname_prefix("foo>=3"), ("foo", ">=3"));
}

#[test]
fn test_extract_pkgname_prefix_whole() {
    assert_eq!(extract_pkgname_prefix("foo"), ("foo", ""));
}

#[test]
fn test_extract_pkgname_prefix_empty() {
    assert_eq!(extract_pkgname_prefix(">=3"), ("", ">=3"));
}
