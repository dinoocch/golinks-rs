#!/bin/bash
if [[ $EUID -ne 0 ]]; then
	SUDOCMD=sudo
else
	SUDOCMD=
fi

${SUDOCMD} mkdir /etc/sv/golinks
${SUDOCMD} sudo cp sv/run /etc/sv/golinks/
${SUDOCMD} sudo useradd -r golinks -d /var/lib/golinks
${SUDOCMD} sudo cp scripts/add_link /usr/local/bin
cargo build --release
${SUDOCMD} sudo cp target/release/golinks-rs /usr/local/bin
