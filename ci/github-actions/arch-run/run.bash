#! /bin/bash
set -o errexit -o pipefail -o nounset

echo '::group::Patching some file...'
cp -v patches/makepkg /usr/bin/makepkg
echo '::endgroup::'

eval "$INPUT_COMMAND"
