#!/bin/bash

version=$1

# bump NPM package versions
find \
    -name "package.json" -and -not -path "./**/node_modules/*" \
    -exec sed -i "s/\"version\": \".*\"/\"version\": \"$1\"/" {} \;

# bump Cargo package versions
# NOTE: version number is guaranteed to be first
find \
    -name "Cargo.toml" \
    -exec sed -i "0,/version = \".*\"/s/version = \".*\"$/version = \"$1\"/" {} \;