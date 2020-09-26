from archlinux/base

run pacman -Syu --noconfirm
run pacman -S --noconfirm --needed --overwrite '*' bash rustup

copy run.bash /run.bash

entrypoint ["/run.bash"]
