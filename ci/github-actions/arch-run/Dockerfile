from archlinux:base-devel

copy pacman.conf /etc/pacman.conf

run pacman -Syu --noconfirm
run pacman -S --noconfirm --needed --overwrite '*' bash rustup pkgconf

copy run.bash /run.bash

entrypoint ["/run.bash"]
