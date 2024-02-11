#!/bin/bash

# Replace this URL with the actual URL of your Rusty Read binary release
# REMEMBER TO UPDATE THE VERSION NUMBER IN THE URL
DOWNLOAD_URL="https://github.com/charlesinwald/rusty_read/releases/download/1.0.1/rusty_read"

INSTALL_DIR="$HOME/.local/bin"
BIN_NAME="rusty_read"
INSTALL_PATH="$INSTALL_DIR/$BIN_NAME"

echo "Installing Rusty Read..."

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Download Rusty Read binary
echo "Downloading Rusty Read..."
curl -L "$DOWNLOAD_URL" -o "$INSTALL_PATH"

# Make the Rusty Read binary executable
chmod +x "$INSTALL_PATH"

# Check if install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Adding $INSTALL_DIR to PATH in your shell configuration file"
    for SH_CONFIG in "$HOME/.bashrc" "$HOME/.zshrc"; do
        if [ -f "$SH_CONFIG" ]; then
            # Avoid duplicate entries
            grep -qxF "export PATH=\$PATH:$INSTALL_DIR" "$SH_CONFIG" || echo "export PATH=\$PATH:$INSTALL_DIR" >> "$SH_CONFIG"
        fi
    done
else
    echo "$INSTALL_DIR is already in PATH"
fi

echo "Installation completed. Please restart your shell or source your configuration file to use rusty_read."
