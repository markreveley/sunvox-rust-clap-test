#!/bin/bash
# Script to build and bundle the CLAP plugin

set -e

echo "Building SunVox CLAP plugin..."
cargo build --release

echo "Creating CLAP bundle..."
BUNDLE_DIR="target/release/sunvox_clap.clap"
rm -rf "$BUNDLE_DIR"

# Detect platform and create appropriate bundle structure
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS: Create proper bundle structure
    echo "Creating macOS bundle structure..."
    mkdir -p "$BUNDLE_DIR/Contents/MacOS"

    # Copy the dylib with the correct name
    cp target/release/libsunvox_clap.dylib "$BUNDLE_DIR/Contents/MacOS/sunvox_clap"

    # Bundle SunVox library with the plugin
    ARCH=$(uname -m)
    if [[ "$ARCH" == "arm64" ]]; then
        SUNVOX_LIB="sunvox_lib/sunvox_lib/macos/lib_arm64/sunvox.dylib"
    else
        SUNVOX_LIB="sunvox_lib/sunvox_lib/macos/lib_x86_64/sunvox.dylib"
    fi
    cp "$SUNVOX_LIB" "$BUNDLE_DIR/Contents/MacOS/sunvox.dylib"

    # Update install names to use @loader_path (relative to plugin binary)
    install_name_tool -change "sunvox.dylib" "@loader_path/sunvox.dylib" "$BUNDLE_DIR/Contents/MacOS/sunvox_clap"

    # Create Info.plist
    cat > "$BUNDLE_DIR/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>sunvox_clap</string>
    <key>CFBundleIdentifier</key>
    <string>com.sunvox.clap-plugin</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>SunVox CLAP</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>CFBundleSignature</key>
    <string>SVXC</string>
    <key>NSPrincipalClass</key>
    <string></string>
</dict>
</plist>
EOF

    INSTALL_DIR="$HOME/Library/Audio/Plug-Ins/CLAP"
    echo "✓ macOS bundle created successfully!"
else
    # Linux: Simple structure
    echo "Creating Linux bundle structure..."
    mkdir -p "$BUNDLE_DIR"
    cp target/release/libsunvox_clap.so "$BUNDLE_DIR/sunvox_clap.so"

    INSTALL_DIR="$HOME/.clap"
    echo "✓ Linux bundle created successfully!"
fi

echo "Plugin location: $BUNDLE_DIR"
echo ""
echo "To install the plugin, run:"
echo "  mkdir -p '$INSTALL_DIR' && cp -r '$BUNDLE_DIR' '$INSTALL_DIR/'"
