use std::fmt::{self, Debug, Formatter};
use topological_sort::TopologicalSort;

#[derive(Clone)]
pub struct PackageBuildOrder(pub TopologicalSort<String>);

impl PackageBuildOrder {
    pub fn insert(&mut self, dependant: String, dependency: String) {
        self.0.add_dependency(dependency, dependant);
    }
}

impl Debug for PackageBuildOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PackageOrder")
    }
}

impl Default for PackageBuildOrder {
    fn default() -> Self {
        PackageBuildOrder(TopologicalSort::new())
    }
}
