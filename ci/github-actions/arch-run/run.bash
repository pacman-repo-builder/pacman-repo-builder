#! /bin/bash
set -o errexit -o pipefail -o nounset

# force makepkg to allow running as root
cp patches/makepkg /usr/bin/makepkg

export LIBRARY_PATH=/usr/lib
export LD_LIBRARY_PATH=/usr/lib

eval "$INPUT_COMMAND"
