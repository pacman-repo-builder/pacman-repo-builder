use serde::Serialize;

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
