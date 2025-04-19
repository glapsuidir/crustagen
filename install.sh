#!/bin/bash

set -e

VERSION="v0.1.1-alpha"
INSTALL_DIR="/usr/local/bin"
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

if [ -f "${HOME}/.bashrc" ]; then
    if ! grep -q "HISTIGNORE='crustagen\*'" "${HOME}/.bashrc"; then
        echo "# Prevent crustagen commands and passwords from being saved in history" >> "${HOME}/.bashrc"
        echo "HISTIGNORE='crustagen*:$HISTIGNORE'" >> "${HOME}/.bashrc"
        echo "Added HISTIGNORE for crustagen to your .bashrc"
    fi
fi

if [ -f "${HOME}/.zshrc" ]; then
    if ! grep -q "function skip_crustagen_from_history()" "${HOME}/.zshrc"; then
        echo "# Prevent crustagen commands and passwords from being saved in history" >> "${HOME}/.zshrc"
        echo "
            autoload -U add-zsh-hook

            function skip_crustagen_from_history() {
                if [[ $1 == crustagen* ]]; then
                    return 1
                fi
            }

            add-zsh-hook zshaddhistory skip_crustagen_from_history
        "
    fi
fi

echo "Launching Crustagen for the first time..."
"$INSTALL_DIR/$BIN_NAME" --first-run

printf "Installation complete!\nYou can run Crustagen by typing 'crustagen'. Try 'crustagen --help' for more info.\n"