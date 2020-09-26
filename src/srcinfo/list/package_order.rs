use std::fmt::{self, Debug, Formatter};
use topological_sort::TopologicalSort;

#[derive(Clone)]
pub struct PackageOrder(pub TopologicalSort<String>);

impl PackageOrder {
    pub fn insert(&mut self, dependant: String, dependency: String) {
        self.0.add_dependency(dependency, dependant);
    }
}

impl Debug for PackageOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PackageOrder")
    }
}

impl Default for PackageOrder {
    fn default() -> Self {
        PackageOrder(TopologicalSort::new())
    }
}
