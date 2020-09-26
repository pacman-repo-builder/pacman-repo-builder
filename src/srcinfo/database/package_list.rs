use std::fmt::{self, Debug, Formatter};
use topological_sort::TopologicalSort;

#[derive(Clone)]
pub struct PackageList(pub TopologicalSort<String>);

impl PackageList {
    pub fn insert(&mut self, dependant: String, dependency: String) {
        self.0.add_dependency(dependency, dependant);
    }
}

impl Debug for PackageList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PackageOrder")
    }
}

impl Default for PackageList {
    fn default() -> Self {
        PackageList(TopologicalSort::new())
    }
}
