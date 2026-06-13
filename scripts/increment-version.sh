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

# 1. Root (blink-md)
update_package "blink-md" "Cargo.toml"

# 2. mcp-core
update_package "mcp-core" "src/mcp/core/Cargo.toml"

# 3. jules
update_package "jules" "src/mcp/jules/Cargo.toml"

# 4. md
update_package "md" "src/mcp/md/Cargo.toml"

# 5. mmd
update_package "mmd" "src/mcp/mmd/Cargo.toml"

echo "==> Successfully incremented all internal package versions by 0.0.1"
