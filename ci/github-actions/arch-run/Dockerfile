from archlinux:base-devel

copy pacman.conf /etc/pacman.conf

run pacman -Syu --noconfirm --needed --overwrite '*' bash rustup gcc llvm pkgconf

copy run.bash /run.bash

entrypoint ["/run.bash"]
