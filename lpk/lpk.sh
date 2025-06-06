#!/bin/bash

output=$(lpk "$@")

if [[ "$output" == cd* ]]; then
    dir=$(echo "$output" | grep '^cd ' | cut -d' ' -f2-)
    cd "$dir"
else
    echo "$output"
fi
