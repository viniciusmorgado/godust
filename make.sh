#!/usr/bin/bash

DIR="$(dirname "$0")"

if cargo "$@"; then
    [ -d "$DIR/target/debug" ] && cp -r "$DIR/src/templates" "$DIR/target/debug/templates"
    [ -d "$DIR/target/release" ] && cp -r "$DIR/src/templates" "$DIR/target/release/templates"
fi
