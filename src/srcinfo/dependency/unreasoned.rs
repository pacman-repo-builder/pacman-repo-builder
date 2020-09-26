use super::reasoned::ReasonedDependency;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct UnreasonedDependency<Name: AsRef<str>>(pub Name);

impl<Name: AsRef<str>> UnreasonedDependency<Name> {
    pub fn into_reasoned_dependency<Reason: AsRef<str>>(
        self,
        reason: Option<Reason>,
    ) -> ReasonedDependency<Name, Reason> {
        ReasonedDependency {
            name: self.0,
            reason,
        }
    }

    pub fn as_str(&self) -> UnreasonedDependency<&str> {
        UnreasonedDependency(self.name())
    }

    pub fn name(&self) -> &str {
        self.0.as_ref()
    }
}
