from archlinux:base-devel

copy pacman.conf /etc/pacman.conf

run pacman -Syu --noconfirm
run ls -lA /var/lib/pacman /var/lib/pacman/*
run pacman -S --noconfirm --needed --overwrite '*' bash rustup gcc llvm pkgconf

copy run.bash /run.bash

entrypoint ["/run.bash"]
