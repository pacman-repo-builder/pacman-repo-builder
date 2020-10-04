use serde::{de::DeserializeOwned, Serialize};

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

pub fn serialize_iter_yaml(
    values: impl IntoIterator<Item = impl Serialize>,
) -> Result<String, serde_yaml::Error> {
    let mut result = String::new();

    for value in values {
        result += serde_yaml::to_string(&value)?.as_str();
        result += "\n";
    }

    Ok(result)
}

pub fn deserialize_multi_docs_yaml<'a, Value>(
    yaml: &'a str,
) -> impl Iterator<Item = Result<Value, serde_yaml::Error>> + 'a
where
    Value: DeserializeOwned + 'a,
{
    yaml.split("\n---\n")
        .filter(|part| !part.trim().is_empty())
        .map(serde_yaml::from_str::<Value>)
}

#[derive(Debug, Copy, Clone)]
pub struct Pair<Primary, Secondary> {
    primary: Primary,
    secondary: Secondary,
}

impl<Primary, Secondary> Pair<Primary, Secondary> {
    pub fn new(primary: Primary, secondary: Secondary) -> Self {
        Pair { primary, secondary }
    }

    pub fn from_tuple((primary, secondary): (Primary, Secondary)) -> Self {
        Pair::new(primary, secondary)
    }

    pub fn into_tuple(self) -> (Primary, Secondary) {
        (self.primary, self.secondary)
    }

    pub fn to_ref(&self) -> Pair<&Primary, &Secondary> {
        Pair::new(&self.primary, &self.secondary)
    }

    pub fn swap_role(self) -> Pair<Secondary, Primary> {
        Pair::new(self.secondary, self.primary)
    }

    pub fn map<Return>(self, f: impl FnOnce(Primary) -> Return) -> Pair<Return, Secondary> {
        Pair::new(f(self.primary), self.secondary)
    }
}

#[cfg(test)]
mod tests;
