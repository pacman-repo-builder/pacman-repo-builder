use super::Version;

#[test]
fn try_to_string_success() {
    let actual = [
        Version::new("0.0.0", "1", "0").try_to_string().unwrap(),
        Version::new("1.0.0", "2", "").try_to_string().unwrap(),
        Version::new("0.1.2", "3", "4").try_to_string().unwrap(),
        Version::new("1.2.3", "4.5", "6").try_to_string().unwrap(),
    ];

    let expected = [
        "0.0.0-1".to_string(),
        "1.0.0-2".to_string(),
        "4:0.1.2-3".to_string(),
        "6:1.2.3-4.5".to_string(),
    ];

    assert_eq!(actual, expected);
}
