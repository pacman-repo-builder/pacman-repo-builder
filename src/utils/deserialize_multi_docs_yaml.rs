use serde::de::DeserializeOwned;

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
