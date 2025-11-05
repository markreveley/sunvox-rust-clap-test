#!/bin/bash
# Script to build and bundle the CLAP plugin

set -e

echo "Building SunVox CLAP plugin..."
cargo build --release

echo "Creating CLAP bundle..."
BUNDLE_DIR="target/release/sunvox_clap.clap"
rm -rf "$BUNDLE_DIR"
mkdir -p "$BUNDLE_DIR"

# Copy the shared library with the correct name for CLAP
cp target/release/libsunvox_clap.so "$BUNDLE_DIR/sunvox_clap.so"

echo "✓ CLAP plugin built successfully!"
echo "Plugin location: $BUNDLE_DIR"
echo ""
echo "To install the plugin, copy the bundle to one of these locations:"
echo "  ~/.clap/"
echo "  /usr/lib/clap/"
echo ""
echo "Example:"
echo "  mkdir -p ~/.clap && cp -r $BUNDLE_DIR ~/.clap/"
