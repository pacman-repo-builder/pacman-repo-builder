use super::Version;

#[test]
fn try_to_string_success() {
    let actual = [
        Version::new("0.0.0", "1", "0"),
        Version::new("0.1.2", "3", "4"),
        Version::new("1.2.3", "4.5", "6"),
    ]
    .iter()
    .map(Version::try_to_string)
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

    let expected = [
        "0.0.0-1".to_string(),
        "4:0.1.2-3".to_string(),
        "6:1.2.3-4.5".to_string(),
    ];

    assert_eq!(actual, expected);
}
