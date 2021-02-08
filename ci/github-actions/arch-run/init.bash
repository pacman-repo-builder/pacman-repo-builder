#! /bin/bash
set -o errexit -o pipefail -o nounset

packages=(
  rustup
  gcc
  llvm
  pkgconf
)

pacman -Syu --noconfirm --needed --overwrite '*' "${packages[@]}"
