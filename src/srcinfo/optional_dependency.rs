use super::dependency::Dependency;

#[derive(Debug, Copy, Clone)]
pub struct OptionalDependency<Name, Reason>
where
    Name: AsRef<str>,
    Reason: AsRef<str>,
{
    pub name: Name,
    pub reason: Option<Reason>,
}

impl<Name, Reason> OptionalDependency<Name, Reason>
where
    Name: AsRef<str>,
    Reason: AsRef<str>,
{
    pub fn into_dependency(self) -> Dependency<Name> {
        Dependency(self.name)
    }

    pub fn as_str(&self) -> OptionalDependency<&str, &str> {
        OptionalDependency {
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

impl<'a> OptionalDependency<&'a str, &'a str> {
    pub fn new(text: &'a str) -> Self {
        let mut parts = text.splitn(1, ':');
        let name = parts.next().unwrap();
        let reason = parts.next().map(|x| x.trim());
        OptionalDependency { name, reason }
    }
}
