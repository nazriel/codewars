#!/bin/bash

set -e

LANGUAGE=$1
shift
TITLE_ESCAPED=$(echo $@ | sed 's/ /-/g' | sed 's/[^a-zA-Z0-9-]//g' | tr '[:upper:]' '[:lower:]')

mkdir -p "$TITLE_ESCAPED"
cd "$TITLE_ESCAPED"

case "$LANGUAGE" in
rust)
	cargo init .
	;;
dlang)
	dub init .
	;;
js)
	npm init -y
	;;
go)
	go mod init "codewars.dzfl.pl/$TITLE_ESCAPED"
	;;
python)
	true
	;;
nasm)
	true
	;;
*)
	echo "Unknown language: $LANGUAGE"
	exit 1
	;;
esac

nvim .
