#!/bin/bash

set -e

INSTALL_DIR="/usr/local/bin"
BINARY_NAME="lpk"
RELEASE_URL="https://github.com/LillyPK/lpk-dir-handler/releases/download/spoonix/lpk"

echo "📥 Downloading lpk from release..."
curl -L "$RELEASE_URL" -o "$BINARY_NAME"

echo "🚚 Moving lpk to $INSTALL_DIR (you may be prompted for sudo)..."
chmod +x "$BINARY_NAME"
sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

echo "✅ lpk installed successfully! You can now run 'lpk' from anywhere."
