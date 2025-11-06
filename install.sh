#!/bin/bash
# Install script for SunVox CLAP plugin on macOS
# This handles the quarantine attribute that macOS adds during file copy

set -e

BUNDLE_DIR="target/release/sunvox_clap.clap"
INSTALL_DIR="$HOME/Library/Audio/Plug-Ins/CLAP"

if [ ! -d "$BUNDLE_DIR" ]; then
    echo "Error: Bundle not found at $BUNDLE_DIR"
    echo "Run ./bundle.sh first to build the plugin"
    exit 1
fi

echo "Installing SunVox CLAP plugin..."
mkdir -p "$INSTALL_DIR"
cp -r "$BUNDLE_DIR" "$INSTALL_DIR/"

# Remove quarantine attribute from the installed library (macOS security)
echo "Removing quarantine attributes..."
xattr -d com.apple.quarantine "$INSTALL_DIR/sunvox_clap.clap/Contents/MacOS/sunvox.dylib" 2>/dev/null || true
xattr -d com.apple.quarantine "$INSTALL_DIR/sunvox_clap.clap/Contents/MacOS/sunvox_clap" 2>/dev/null || true

# Ad-hoc sign the plugin for local development (fixes code signature issues)
echo "Signing plugin for local development..."
codesign --force --sign - "$INSTALL_DIR/sunvox_clap.clap/Contents/MacOS/sunvox_clap" 2>/dev/null || true

echo "✓ Plugin installed successfully to $INSTALL_DIR/sunvox_clap.clap"
echo ""
echo "Next steps:"
echo "  1. Restart your DAW (or rescan plugins)"
echo "  2. Look for 'SunVox CLAP' in the plugin browser"
echo "  3. Load it on a track and test"
