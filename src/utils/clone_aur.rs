use super::super::{manifest::BuildMetadata, srcinfo::SrcInfo};
use super::{
    read_srcinfo_file, read_srcinfo_from_directory, read_srcinfo_from_pkgbuild, AlpmWrapper,
};
use git2::Repository;
use indexmap::IndexSet;
use pipe_trait::*;
use rayon::prelude::*;
use std::{ops::Add, path::Path};

#[derive(Debug)]
pub struct CloneAur<'a> {
    pub container: &'a Path,
    pub read_build_metadata: BuildMetadata,
    pub package_names: &'a [String],
    pub installed_dependencies: IndexSet<String>,
    pub alpm: AlpmWrapper,
}

impl<'a> CloneAur<'a> {
    pub fn run(self) -> CloneAurEffect {
        let CloneAur {
            container,
            read_build_metadata,
            package_names,
            installed_dependencies,
            alpm,
        } = self;

        let effect = package_names
            .par_iter()
            .flat_map(|package_name| {
                let directory = container.join(package_name);
                if directory.exists() {
                    eprintln!("🛈 Skip {:?} (already exists)", directory);
                    return None;
                }

                let url = format!("https://aur.archlinux.org/{}.git", package_name);
                if let Err(error) = Repository::clone(&url, &directory) {
                    eprintln!(
                        "⮾ Failed to clone {:?} into {:?}: {}",
                        url, package_name, error,
                    );
                    return Some(Err(()));
                }
                eprintln!("🛈 Cloned {:?} from {:?}", package_name, url);

                Some(Ok((package_name, directory)))
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|item| {
                item.and_then(|(package_name, directory)| {
                    let mut added_package_names = IndexSet::new();
                    added_package_names.insert(package_name.to_string());

                    let missing_dependencies: IndexSet<_> = match read_build_metadata {
                        BuildMetadata::SrcInfo => {
                            directory.join(".SRCINFO").pipe(read_srcinfo_file)
                        }
                        BuildMetadata::PkgBuild => read_srcinfo_from_pkgbuild(&directory),
                        BuildMetadata::Either => read_srcinfo_from_directory(&directory),
                    }
                    .map_err(|error| {
                        eprintln!("{}", error);
                    })?
                    .pipe(SrcInfo)
                    .all_required_dependencies()
                    .filter(|x| !contains_str(package_names.iter(), x.name))
                    .filter(|x| !contains_str(installed_dependencies.iter(), x.name))
                    .filter(|x| !alpm.is_provided(x.name))
                    .map(|x| x.name.to_string())
                    .collect();

                    Ok(CloneAurEffect {
                        added_package_names,
                        missing_dependencies,
                        error_count: 0,
                    })
                })
            })
            .map(|effect| {
                effect.unwrap_or_else(|()| CloneAurEffect {
                    error_count: 1,
                    ..Default::default()
                })
            })
            .fold(CloneAurEffect::default(), Add::add);

        if effect.missing_dependencies.is_empty() {
            return effect;
        }

        let mut next_installed_dependencies = installed_dependencies;
        next_installed_dependencies.extend(package_names.to_vec());
        let next_package_names: Vec<_> = effect.missing_dependencies.into_iter().collect();
        let mut next_effect = CloneAur {
            container,
            read_build_metadata,
            alpm,
            installed_dependencies: next_installed_dependencies,
            package_names: &next_package_names,
        }
        .run();

        assert!(
            next_effect.missing_dependencies.is_empty(),
            "no remaining missing dependencies",
        );

        CloneAurEffect {
            missing_dependencies: next_effect.missing_dependencies,
            added_package_names: {
                next_effect
                    .added_package_names
                    .extend(effect.added_package_names);
                next_effect.added_package_names
            },
            error_count: next_effect.error_count + effect.error_count,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct CloneAurEffect {
    missing_dependencies: IndexSet<String>,
    pub added_package_names: IndexSet<String>,
    pub error_count: u32,
}

impl Add for CloneAurEffect {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        macro_rules! sum_set {
            ($name:ident) => {{
                let mut sum = self.$name;
                sum.extend(other.$name);
                sum
            }};
        }

        CloneAurEffect {
            missing_dependencies: sum_set!(missing_dependencies),
            added_package_names: sum_set!(added_package_names),
            error_count: self.error_count + other.error_count,
        }
    }
}

fn contains_str<Container>(container: Container, item: &str) -> bool
where
    Container: IntoIterator,
    Container::Item: AsRef<str>,
{
    container.into_iter().any(|x| x.as_ref() == item)
}
