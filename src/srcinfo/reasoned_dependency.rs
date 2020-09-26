use super::unreasoned_dependency::UnreasonedDependency;

#[derive(Debug, Copy, Clone)]
pub struct ReasonedDependency<Name, Reason>
where
    Name: AsRef<str>,
    Reason: AsRef<str>,
{
    pub name: Name,
    pub reason: Option<Reason>,
}

impl<Name, Reason> ReasonedDependency<Name, Reason>
where
    Name: AsRef<str>,
    Reason: AsRef<str>,
{
    pub fn into_unreasoned_dependency(self) -> UnreasonedDependency<Name> {
        UnreasonedDependency(self.name)
    }

    pub fn as_str(&self) -> ReasonedDependency<&str, &str> {
        ReasonedDependency {
            name: self.name(),
            reason: self.reason(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn reason(&self) -> Option<&str> {
        if let Some(reason) = &self.reason {
            Some(reason.as_ref())
        } else {
            None
        }
    }
}

impl<'a> ReasonedDependency<&'a str, &'a str> {
    pub fn new(text: &'a str) -> Self {
        let mut parts = text.splitn(1, ':');
        let name = parts.next().unwrap();
        let reason = parts.next().map(|x| x.trim());
        ReasonedDependency { name, reason }
    }
}
