use itertools::Itertools;
use serde::Serialize;

pub fn split_str_once(text: &str, when: impl Fn(char, usize) -> bool) -> (&str, &str) {
    for (index, current_char) in text.char_indices() {
        if when(current_char, index) {
            return (&text[0..index], &text[index..]);
        }
    }

    (text, "")
}

pub fn extract_value_from_line<'a>(prefix: &str, line: &'a str) -> Option<&'a str> {
    let line = line.trim();
    if !line.starts_with(prefix) {
        return None;
    }
    let line = &line[prefix.len()..].trim_start();
    if !line.starts_with('=') {
        return None;
    }
    Some(line[1..].trim())
}

pub fn extract_pkgname_prefix(text: &str) -> (&str, &str) {
    split_str_once(text, |current_char, _| match current_char {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '@' | '.' | '_' | '+' | '-' => false,
        _ => true,
    })
}

pub fn serialize_iter_yaml(values: impl IntoIterator<Item = impl Serialize>) -> String {
    values
        .into_iter()
        .map(|value| serde_yaml::to_string(&value))
        .map(Result::unwrap)
        .join("\n")
}

#[cfg(test)]
mod tests;
