from archlinux/base

run pacman -Syu --noconfirm
run pacman -S --noconfirm --needed --overwrite '*' bash rustup gcc llvm 'python>=3.8'

copy run.bash /run.bash

entrypoint ["/run.bash"]
