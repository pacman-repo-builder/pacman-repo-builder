#! /bin/bash
set -o errexit -o pipefail -o nounset

# force makepkg to allow running as root
cp patches/makepkg /usr/bin/makepkg

eval "$INPUT_COMMAND"
