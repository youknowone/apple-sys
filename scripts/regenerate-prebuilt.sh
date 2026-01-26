#!/bin/bash
set -euo pipefail

# Regenerate prebuilt macOS bindings from the local SDK using bindgen.
#
# This builds apple-sys with all macOS frameworks enabled via bindgen,
# then copies the generated .rs files to the prebuilt crate.
#
# Prerequisites:
#   - Xcode with macOS SDK installed
#   - Rust toolchain with bindgen support
#
# Usage:
#   bash scripts/regenerate-prebuilt.sh
#
# After running, you should also run configure.py to update Cargo.toml
# feature dependencies from the new prebuilt imports:
#   cd crates/apple-sys && python configure.py

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
APPLE_SYS="$REPO_ROOT/crates/apple-sys"
PREBUILT_DIR="$REPO_ROOT/crates/apple-sys-prebuilt-macosx/generated"

# Extract macOS framework names from macos.inc.rs
FEATURES=$(grep -oE 'feature = "\w+"' "$APPLE_SYS/macos.inc.rs" | sed 's/feature = "//;s/"//' | paste -sd, -)

echo "Building apple-sys with bindgen for all macOS frameworks..."
echo "  Features: $(echo "$FEATURES" | tr ',' '\n' | wc -l | tr -d ' ') frameworks"

cargo build \
    --manifest-path="$APPLE_SYS/Cargo.toml" \
    --features "bindgen,$FEATURES" \
    --release 2>&1 | tail -20

# Find the OUT_DIR from the build
OUT_DIR=$(find "$REPO_ROOT/target/release/build" -path "*/apple-sys-*/out" -type d | head -1)

if [ -z "$OUT_DIR" ]; then
    echo "ERROR: Could not find OUT_DIR. Build may have failed."
    exit 1
fi

echo ""
echo "OUT_DIR: $OUT_DIR"
echo "Copying generated files to $PREBUILT_DIR ..."

# Count files
GENERATED=$(ls "$OUT_DIR"/*.rs 2>/dev/null | wc -l | tr -d ' ')
echo "  $GENERATED .rs files generated"

# Copy all .rs files
mkdir -p "$PREBUILT_DIR"
cp "$OUT_DIR"/*.rs "$PREBUILT_DIR/"

echo "  Copied to $PREBUILT_DIR"
echo ""
echo "Done. Next steps:"
echo "  1. cd crates/apple-sys && python configure.py"
echo "  2. Review changes with: git diff --stat"
