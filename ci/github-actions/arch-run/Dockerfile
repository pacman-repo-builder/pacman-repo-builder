from archlinux:base-devel-20210110.0.13332

copy pacman.conf /etc/pacman.conf

run pacman -Syu --noconfirm
run pacman -S --noconfirm --needed --overwrite '*' bash rustup gcc llvm pkgconf

copy run.bash /run.bash

entrypoint ["/run.bash"]
