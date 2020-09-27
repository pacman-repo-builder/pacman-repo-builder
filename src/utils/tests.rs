use super::{extract_pkgname_prefix, extract_value_from_line};

#[test]
fn extract_value_from_line_some() {
    assert_eq!(
        extract_value_from_line("pkgname", "  pkgname = foo  "),
        Some("foo"),
    );
}

#[test]
fn extract_value_from_line_none() {
    assert_eq!(
        extract_value_from_line("pkgname", "  pkgbase = foo  "),
        None,
    );
}

#[test]
fn extract_pkgname_prefix_partial() {
    assert_eq!(extract_pkgname_prefix("foo>=3"), ("foo", ">=3"));
}

#[test]
fn extract_pkgname_prefix_whole() {
    assert_eq!(extract_pkgname_prefix("foo"), ("foo", ""));
}

#[test]
fn extract_pkgname_prefix_empty() {
    assert_eq!(extract_pkgname_prefix(">=3"), ("", ">=3"));
}
