#! /bin/bash
set -o errexit -o pipefail

FMT_UPDATE="${FMT_UPDATE:-false}"
cargo_fmt_flag=()

case "$FMT_UPDATE" in
true)
	cargo_fmt_flag=()
	;;
false)
	cargo_fmt_flag=('--check')
	;;
*)
	echo "error: \$FMT_UPDATE is neither 'true' or 'false': '$FMT_UPDATE'" >/dev/stderr
	exit 1
	;;
esac

cargo fmt -- "${cargo_fmt_flag[@]}"
