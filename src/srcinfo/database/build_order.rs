use super::Database;
use indexmap::IndexMap;
use petgraph::{algo::toposort, graph::Graph};
use pipe_trait::*;
use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    path::Path,
};

impl<PkgBase, PkgName, SrcInfoContent, BuildDir>
    Database<PkgBase, PkgName, SrcInfoContent, BuildDir>
where
    PkgBase: AsRef<str> + Hash + Eq + Clone,
    PkgName: AsRef<str> + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str>,
    BuildDir: AsRef<Path>,
{
    pub fn build_order(&self) -> Result<impl Iterator<Item = &PkgBase>, BuildOrderError<&PkgBase>> {
        let mut graph = Graph::<&PkgBase, ()>::new();
        let mut pkgbase_to_node_index = IndexMap::new();

        // Register pkgbase as node indices
        for (pkgbase, _) in self.dependencies() {
            let node_index = graph.add_node(pkgbase);
            pkgbase_to_node_index.insert(pkgbase, node_index);
        }

        // Register dependency links as node edges
        for (dependant, dependencies) in self.dependencies() {
            let dependant_index = pkgbase_to_node_index
                .get(dependant)
                .copied()
                .expect("get index of dependant");
            for dependency in dependencies {
                let dependency_index = pkgbase_to_node_index
                    .get(dependency)
                    .copied()
                    .expect("get index of dependency");
                graph.add_edge(dependency_index, dependant_index, ());
            }
        }

        toposort(&graph, None)
            .map_err(|cycle| cycle.node_id())
            .map_err(|index| *graph.node_weight(index).expect("get cyclic point"))
            .map_err(BuildOrderError::CyclicDependency)?
            .into_iter()
            .filter_map(move |index| graph.node_weight(index).copied())
            .pipe(Ok)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BuildOrderError<PkgBase> {
    CyclicDependency(PkgBase),
}

impl<PkgBase> Display for BuildOrderError<PkgBase>
where
    PkgBase: Display,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BuildOrderError::CyclicDependency(pkgbase) => {
                write!(formatter, "dependency cycle detected at {}", pkgbase)
            }
        }
    }
}
