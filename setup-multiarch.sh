#!/bin/bash

# Setup script for installing aarch64 libcamera in WSL

echo "=== Setting up multi-arch support for aarch64 ==="

# Add aarch64 architecture
echo "Adding arm64 architecture..."
sudo dpkg --add-architecture arm64

# Update package lists
echo "Updating package lists..."
sudo apt update

# Install libcamera-dev for aarch64
echo "Installing libcamera-dev:arm64..."
sudo apt install -y libcamera-dev:arm64

# Install cross-compilation toolchain
echo "Installing cross-compilation tools..."
sudo apt install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu pkg-config

# Verify installation
echo ""
echo "=== Verification ==="
if [ -f /usr/lib/aarch64-linux-gnu/pkgconfig/libcamera.pc ]; then
    echo "✓ libcamera.pc found at /usr/lib/aarch64-linux-gnu/pkgconfig/libcamera.pc"
    pkg-config --modversion libcamera && echo "✓ libcamera version check successful"
else
    echo "✗ libcamera.pc not found - there may be an issue"
fi

echo ""
echo "Setup complete!"
echo "Now update your .cargo/config.toml with:"
echo ""
echo "[env]"
echo "PKG_CONFIG_ALLOW_CROSS = \"1\""
echo "PKG_CONFIG_PATH = \"/usr/lib/aarch64-linux-gnu/pkgconfig\""
