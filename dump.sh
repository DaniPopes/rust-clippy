#!/usr/bin/env bash
set -eo pipefail

rm -rf dump2
mkdir dump2
cp mir_dump/$1.*.mir dump2

if [ -n "$2" ]; then
    for file in dump2/*.mir; do
        rg "$2" "$file" >/dev/null || echo "$file";
    done | sort
else
    ls -lah dump2
fi
