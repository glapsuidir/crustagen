#!/bin/bash

set -e

VERSION="v0.1.1-alpha"

echo "Fetching the latest release (including pre-releases) from GitHub..."
echo "Latest version: $VERSION"

RELEASE_URL="https://github.com/glapsuidir/crustagen/releases/download/v0.1.0-alpha/crustagen"

if [ "$VERSION" == "null" ]; then
    echo "Error: No valid release version found. Exiting."
    exit 1
fi

echo "Downloading $VERSION..."
curl -L $RELEASE_URL -o crustagen

chmod +x crustagen

INSTALL_DIR="/usr/local/bin"

if [ ! -f "$INSTALL_DIR/crustagen" ]; then
    sudo mv crustagen "$INSTALL_DIR/"

    sudo chmod +x "$INSTALL_DIR/crustagen"
    echo "Installation complete! You can run Crustagen by typing in 'crustagen' followed by any relevant flags. Try crustagen --help for more info."
else
    echo "Crustagen is already installed in $INSTALL_DIR"
fi

sudo mv -f crustagen /usr/local/bin/crustagen