#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct UnreasonedDependency<Name: AsRef<str>>(pub Name);

impl<Name: AsRef<str>> UnreasonedDependency<Name> {
    pub fn as_str(&self) -> UnreasonedDependency<&str> {
        UnreasonedDependency(self.name())
    }

    pub fn name(&self) -> &str {
        self.0.as_ref()
    }
}
