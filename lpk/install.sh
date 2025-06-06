#!/bin/bash

set -e

REPO_URL="https://github.com/youruser/lpk"
BINARY_URL="$REPO_URL/releases/latest/download/lpk"
INSTALL_PATH="/usr/local/bin"

echo "Downloading lpk binary..."
curl -L "$BINARY_URL" -o lpk
chmod +x lpk
sudo mv lpk "$INSTALL_PATH/lpk"

echo "Installing shell wrapper..."
curl -L "$REPO_URL/raw/main/lpk.sh" -o lpk.sh
chmod +x lpk.sh
sudo mv lpk.sh "$INSTALL_PATH/lpk.sh"

echo
echo "To use lpk, add this function to your shell:"
echo
cat <<EOF
lpk() {
    source /usr/local/bin/lpk.sh "\$@"
}
EOF
echo
echo "Add it to ~/.bashrc or ~/.zshrc for permanent use."
