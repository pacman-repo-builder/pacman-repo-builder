use super::extract_value_from_line;

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
