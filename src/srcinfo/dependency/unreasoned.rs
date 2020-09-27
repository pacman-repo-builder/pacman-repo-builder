use super::super::super::utils::extract_pkgname_prefix;
use super::reasoned::ReasonedDependency;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct UnreasonedDependency<Name, Range>
where
    Name: AsRef<str>,
    Range: AsRef<str>,
{
    pub name: Name,
    pub range: Range,
}

impl<Name, Range> UnreasonedDependency<Name, Range>
where
    Name: AsRef<str>,
    Range: AsRef<str>,
{
    pub fn into_reasoned_dependency<Reason: AsRef<str>>(
        self,
        reason: Option<Reason>,
    ) -> ReasonedDependency<Name, Range, Reason> {
        ReasonedDependency {
            name: self.name,
            range: self.range,
            reason,
        }
    }

    pub fn as_str(&self) -> UnreasonedDependency<&str, &str> {
        UnreasonedDependency {
            name: self.name(),
            range: self.range(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn range(&self) -> &str {
        self.range.as_ref()
    }
}

impl<'a> UnreasonedDependency<&'a str, &'a str> {
    pub fn new(text: &'a str) -> Self {
        let (name, range) = extract_pkgname_prefix(text);
        UnreasonedDependency { name, range }
    }
}
