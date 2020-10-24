# PacMan Repo Builder

[![Test](https://github.com/KSXGitHub/pacman-repo-builder/workflows/Test/badge.svg)](https://github.com/KSXGitHub/pacman-repo-builder/actions?query=workflow%3ATest)
[![Crates.io Version](https://img.shields.io/crates/v/pacman-repo-builder?logo=rust)](https://crates.io/crates/pacman-repo-builder)

Build a custom pacman repository from a collection of PKGBUILD directories.

## Usage

**⚠ WARNING:** This program is meant to be used within a docker container.

### Manifest file

Manifest file is always named `build-pacman-repo.yaml`. It contains instruction to build a pacman repository.

**Example Manifest File:**

```yaml
# build-pacman-repo.yaml
global-settings:
  repository: repo/repo.db.tar.gz
  container: container
  read-build-metadata: either
  install-missing-dependencies: false
  clean-before-build: false
  clean-after-build: false
  force-rebuild: true
  pacman: pacman
  packager: Bob <bob@example.com>
  allow-failure: true
  dereference-database-symlinks: true
members:
  - directory: foo
  - directory: bar
    read-build-metadata: pkgbuild
    clean-before-build: false
    force-rebuild: true
    allow-failure: false
  - directory: bar
    install-missing-dependencies: true
    clean-after-build: false
    pacman: yay
  - directory: baz
    read-build-metadata: srcinfo
    install-missing-dependencies: false
    clean-before-build: true
    clean-after-build: false
    force-rebuild: true
    pacman: yay
    allow-failure: false
```

**Field Explanations:**

_Distinct Fields:_

| Field                           | Required/Optional                      | Description                                                                                                                        |
|---------------------------------|----------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|
| `global-settings`               | required                               | Includes global settings from which all members inherit from.<br>Some settings can be overwritten by member customized properties. |
| `member`                        | optional, default = `[]`               | List all members.                                                                                                                  |
| `global-settings.repository`    | required                               | Path to repository file (typically ends with `.db.tar.gz`).<br>It will be passed to `repo-add` command after each build.           |
| `global-settings.container`     | optional, default = `.`                | Directory that contains all build directories (a.k.a. members).                                                                    |
| `global-settings.arch-filter`   | optional, default = `any`              | Specify all CPU architectures to build.<br>Either `any` or an array of strings (e.g. `[x86_64, i686]`).                            |
| `global-settings.packager`      | optional, default = `Unknown Packager` | Identity of person or entity that produces the packages (i.e. the one who run this program).                                       |
| `dereference-database-symlinks` | optional, default = `false`            | If `true`, all `*.db` and `*.files` symlinks will be converted to real files.                                                      |
| `member.directory`              | required                               | Path to build directory of each member (relative to `global-settings.container`).                                                  |

_Shared Fields:_ Fields that exist in both `global-settings` and `member`. If `global-settings` and `member` both contain a field, `member`'s field will be prioritized.

| Field                          | Default<br>(`global-settings`) | Description                                                                                                                                                                                                           |
|--------------------------------|--------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `read-build-metadata`          | `either`                       | Where to read build metadata:<br>`srcinfo`: From `.SRCINFO` file.<br>`pkgbuild`: From output of `makepkg --printsrcinfo`.<br>`either`: From `.SRCINFO` file if it exists, otherwise execute `makepkg --printsrcinfo`. |
| `install-missing-dependencies` | `false`                        | Install packages found in `depends` and `makedepends` before each build.                                                                                                                                              |
| `clean-before-build`           | `false`                        | Clean `$srcdir` and `$pkgdir` before each build.                                                                                                                                                                      |
| `clean-after-build`            | `false`                        | Clean up after each build.                                                                                                                                                                                            |
| `force-rebuild`                | `false`                        | Force build even if target package already exists.                                                                                                                                                                    |
| `pacman`                       | `pacman`                       | Package manager program to use.<br>The program must recognize `pacman`'s CLI arguments and options.                                                                                                                   |
| `allow-failure`                | `false`                        | If `false`, exits immediately when a build fails.<br>If `true`, ignore build failure should one occurs.                                                                                                               |

### Generate manifest file

Listing every member in a manifest file can be a chore. So when there are no members with customized properties, you can generate the manifest file the reflect the build directories instead:

```sh
build-pacman-repo print-config \
  --repository $repo_dir/$repo_name.db.tar.gz \
  --container build-directories \
  --require-pkgbuild \
  --require-srcinfo \
  --with-install-missing-dependencies true \
  > build-pacman-repo.yaml
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
build-pacman-repo build
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

## GitHub Action

[pacman-repo-builder/action](https://github.com/pacman-repo-builder/action) let you run `build-pacman-repo` on an Arch Linux container.

## License

[GPLv3](https://git.io/JTBVM) © [Hoàng Văn Khải](https://ksxgithub.github.io).
