pub fn outdated_packages<'a, Latest: ToString>(
    latest_packages: impl IntoIterator<Item = Latest> + 'a,
    current_packages: &'a [String],
) -> impl Iterator<Item = (String, Latest)> + 'a {
    latest_packages
        .into_iter()
        .map(|latest| (latest.to_string(), latest))
        .filter(move |(filename, _)| !current_packages.contains(filename))
}
