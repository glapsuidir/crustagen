#!/bin/bash

set -e

VERSION="v0.1.1-alpha"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/crustagen"
BIN_NAME="crustagen"
RELEASE_URL="https://github.com/glapsuidir/crustagen/releases/download/$VERSION/$BIN_NAME"

echo "Fetching the latest release from GitHub..."
echo "Latest version: $VERSION"

TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "Downloading $VERSION..."
curl -fL "$RELEASE_URL" -o "$BIN_NAME" || {
    echo "Failed to download binary. Does the release asset exist at $RELEASE_URL?"
    exit 1
}

chmod +x "$BIN_NAME"

if [ -f "$INSTALL_DIR/$BIN_NAME" ]; then
    echo "Crustagen is already installed in $INSTALL_DIR. Overwriting..."
    sudo rm "$INSTALL_DIR/$BIN_NAME"
fi

sudo mv "$BIN_NAME" "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/$BIN_NAME"

sudo mkdir -p "$CONFIG_DIR"

echo "initialized=true" | sudo tee "$CONFIG_DIR/config.txt" > /dev/null

HISTIGNORE='crustagen*'

printf "Installation complete!\nYou can run Crustagen by typing 'crustagen'. Try 'crustagen --help' for more \ninfo.\n"
printf "Configuration file created at: $CONFIG_DIR/config.txt\n"
