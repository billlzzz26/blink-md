#!/bin/bash
# Reusable Version Bumper and File Integrity Script
set -e

NEW_VERSION=$1
PACKAGE_NAME=$2

if [ -z "$NEW_VERSION" ]; then
    echo "Usage: $0 <version> [package_name]"
    echo "Example: $0 0.0.2 blink-md"
    exit 1
fi

if [ -n "$PACKAGE_NAME" ]; then
    echo "==> Updating version to $NEW_VERSION for package: $PACKAGE_NAME..."
    # Find the Cargo.toml for the specific package
    FILE=$(grep -l "name = \"$PACKAGE_NAME\"" $(find . -name "Cargo.toml" -not -path "*/target/*"))
    if [ -n "$FILE" ]; then
        sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$FILE"
        echo "Updated $FILE"
    else
        echo "Error: Package '$PACKAGE_NAME' not found."
        exit 1
    fi
else
    echo "==> Updating version to $NEW_VERSION in Cargo.toml..."
    # Single-crate project: only the root Cargo.toml carries a version.
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
fi

echo "==> Standardizing line endings (LF) and ensuring trailing newlines..."

# Find all source and text files, convert to LF and ensure trailing newline
find . -type f \( -name "*.rs" -o -name "*.toml" -name "*.md" -o -name "*.sh" -o -name "*.json" \) -not -path "*/target/*" -not -path "*/.git/*" | while read -r file; do
    # Convert CRLF to LF
    sed -i 's/\r$//' "$file"
    # Ensure trailing newline if missing
    if [ -n "$(tail -c 1 "$file")" ]; then
        echo "" >> "$file"
    fi
done

echo "==> Done."
