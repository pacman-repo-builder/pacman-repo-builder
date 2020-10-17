# PacMan Repo Builder

[![Test](https://github.com/KSXGitHub/pacman-repo-builder/workflows/Test/badge.svg)](https://github.com/KSXGitHub/pacman-repo-builder/actions?query=workflow%3ATest)
[![Crates.io Version](https://img.shields.io/crates/v/pacman-repo-builder?logo=rust)](https://crates.io/crates/pacman-repo-builder)

Build a custom pacman repository from a collection of PKGBUILD directories.

## Usage

**⚠ WARNING:** This program is meant to be used within a docker container.

### Generate manifest file

Manifest file is always named `build-pacman-repo.yaml`. It contains instruction to build a pacman repository.

```sh
build-pacman-repo print-config -T $repo_dir/$repo_name.db.tar.gz -D build-directories > build-pacman-repo.yaml
```

_Note:_ Replace `$repo_dir` with path of your repository directory. This directory would contains all built packages.
_Note:_ Replace `$repo_name` with name of your repository file. This file would be fetched by `pacman` to check for updates.

### Replace `/usr/bin/makepkg` with one that allows running as root

The normal `makepkg` script does not allow running as root. While it may make sense in a user's machine, it inconveniences a Docker container.

```sh
build-pacman-repo patch-makepkg --replace
```

### Build a pacman repositories

```sh
build-pacman-repo build --syncdeps
```

_Note:_ Make sure that `build-pacman-repo.yaml` file exists in current working directory.

### Print help message

```sh
build-pacman-repo help
```

```sh
build-pacman-repo --help
```

```sh
build-pacman-repo help $command
```

```sh
build-pacman-repo $command --help
```

## License

This program contains [a modification of `makepkg` script](https://git.io/JTqsZ) which is released under [GNU Public License Version 3](https://git.io/JTq3Y). See content of `makepkg` for authors and contributors.

Everything else is released under [MIT](https://git.io/JTq3K). Copyright © 2020 [Hoàng Văn Khải](https://github.com/KSXGitHub/).
