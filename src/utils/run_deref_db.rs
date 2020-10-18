use std::{
    fs::{canonicalize, copy, read_dir, remove_file},
    io,
    path::Path,
};

pub fn run_deref_db(repository_directory: &Path) -> Result<(), io::Error> {
    let canon_repository_directory =
        canonicalize(repository_directory).expect("canonicalize repository directory");
    for entry in read_dir(repository_directory).expect("read repository repository") {
        let entry = entry.expect("read entry");
        if !entry
            .file_type()
            .map(|kind| kind.is_symlink())
            .unwrap_or(false)
        {
            continue;
        }
        let file_name = entry.file_name();
        let lossy_file_name = file_name.to_string_lossy();
        if !lossy_file_name.ends_with(".db") && !lossy_file_name.ends_with(".files") {
            continue;
        }
        let link_path = canon_repository_directory.join(file_name);
        let link_target = canonicalize(&link_path).expect("canonicalize suspect");
        eprintln!("  → Delete {:?}", &link_path);
        remove_file(&link_path)?;
        eprintln!("  → Copy {:?} to {:?}", link_target, &link_path);
        copy(link_target, link_path)?;
    }

    Ok(())
}
