use super::AlpmWrapper;
use alpm::PackageReason;
use indexmap::{IndexMap, IndexSet};

pub fn orphan_packages(alpm_wrapper: &AlpmWrapper) -> IndexSet<String> {
    let mut tree: IndexMap<String, IndexSet<String>> = alpm_wrapper
        .installed_packages()
        .filter(|pkg| pkg.reason() == PackageReason::Depend)
        .map(|pkg| (pkg.name().to_string(), IndexSet::new()))
        .collect();

    loop {
        let mut end_loop = true;

        for pkg in alpm_wrapper.installed_packages() {
            for dependency in pkg.depends() {
                if let Some(dependant_list) = tree.get_mut(dependency.name()) {
                    if dependant_list.insert(pkg.name().to_string()) {
                        end_loop = false;
                    }
                }
            }
        }

        if end_loop {
            break;
        }
    }

    loop {
        let mut end_loop = true;

        for dependency in tree.keys().cloned().collect::<Vec<_>>() {
            for dependant_list in tree.values_mut() {
                if dependant_list.remove(&dependency) {
                    end_loop = false;
                }
            }
        }

        if end_loop {
            break;
        }
    }

    tree.iter()
        .filter(|(_, dependant_list)| !dependant_list.is_empty())
        .map(|(dependency, _)| dependency.clone())
        .collect()
}
