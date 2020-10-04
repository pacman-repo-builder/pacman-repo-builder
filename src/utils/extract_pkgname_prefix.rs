use super::split_str_once;

pub fn extract_pkgname_prefix(text: &str) -> (&str, &str) {
    split_str_once(text, |current_char, _| match current_char {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '@' | '.' | '_' | '+' | '-' => false,
        _ => true,
    })
}
