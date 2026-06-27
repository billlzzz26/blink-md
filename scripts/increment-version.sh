#!/bin/bash
# Semantic Version Incrementer (Patch level)
set -e

increment_patch() {
    local version=$1
    local major=$(echo $version | cut -d. -f1)
    local minor=$(echo $version | cut -d. -f2)
    local patch=$(echo $version | cut -d. -f3)
    echo "$major.$minor.$((patch + 1))"
}

update_package() {
    local name=$1
    local file=$2
    local current=$(grep "^version =" "$file" | cut -d'"' -f2)
    local next=$(increment_patch "$current")
    echo "Incrementing $name: $current -> $next"
    sed -i "s/^version = \".*\"/version = \"$next\"/" "$file"
}

# Single-crate project: only the root blink-md package has a version.
update_package "blink-md" "Cargo.toml"

echo "==> Successfully incremented the blink-md package version by 0.0.1"
