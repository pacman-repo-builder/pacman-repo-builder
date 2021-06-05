#! /bin/bash
set -o errexit -o pipefail -o nounset

# Patch makepkg
cargo run --bin=build-pacman-repo -- patch-makepkg --replace

eval "$INPUT_COMMAND"
