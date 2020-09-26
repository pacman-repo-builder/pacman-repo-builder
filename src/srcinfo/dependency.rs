#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Dependency<Name: AsRef<str>>(pub Name);

impl<Name: AsRef<str>> Dependency<Name> {
    pub fn as_str(&self) -> Dependency<&str> {
        Dependency(self.name())
    }

    pub fn name(&self) -> &str {
        self.0.as_ref()
    }
}
