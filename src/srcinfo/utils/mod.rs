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
    for (index, current_char) in text.char_indices() {
        match current_char {
            'a'..='z' | 'A'..='Z' | '@' | '.' | '_' | '+' | '-' => continue,
            _ => return (&text[0..index], &text[index..]),
        }
    }

    (text, "")
}

#[cfg(test)]
mod tests;
