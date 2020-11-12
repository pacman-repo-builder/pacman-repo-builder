use super::{extract_pkgname_prefix, split_str_once};

pub fn list_provides_from_info(info: &str) -> impl Iterator<Item = &str> {
    info.lines()
        .map(|line| {
            let (left, right) = split_str_once(line, |x, _| x == ':');
            let left = left.trim_end();
            if right.is_empty() {
                (None, left)
            } else {
                debug_assert_eq!(right.chars().next(), Some(':'));
                (Some(left), &right[1..])
            }
        })
        .skip_while(|(left, _)| *left != Some("Provides"))
        .take_while(|(left, _)| *left == None || *left == Some("Provides"))
        .map(|(_, right)| right.trim_start())
        .filter(|value| *value != "None")
        .flat_map(|value| value.split_whitespace())
        .map(|value| extract_pkgname_prefix(value).0)
}

#[cfg(test)]
mod tests {
    use super::list_provides_from_info;
    use pipe_trait::*;

    macro_rules! test_case {
        ($name:ident, $lines:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let info = $lines.join("\n");
                let actual: Vec<_> = info.as_str().pipe(list_provides_from_info).collect();
                let expected: Vec<&str> = $expected;
                assert_eq!(actual, expected);
            }
        };
    }

    test_case!(
        none,
        vec![
            "Repository             : core",
            "Name                   : foo",
            "Provides               : None",
            "Depends On             : abc",
            "                         def>=1.0",
            "Licenses               : GPL",
            "Architecture           : Any",
        ],
        Default::default()
    );

    test_case!(
        single_line,
        vec![
            "Repository             : core",
            "Name                   : foo",
            "Provides               : bar baz qux",
            "Depends On             : abc",
            "                         def>=1.0",
            "Licenses               : GPL",
            "Architecture           : Any",
        ],
        vec!["bar", "baz", "qux"]
    );

    test_case!(
        multiple_lines,
        vec![
            "Repository             : core",
            "Name                   : foo",
            "Provides               : bar",
            "                         baz",
            "                         qux",
            "Depends On             : abc",
            "                         def>=1.0",
            "Licenses               : GPL",
            "Architecture           : Any",
        ],
        vec!["bar", "baz", "qux"]
    );
}
