#!/bin/bash
# RustX Installation Script
# Downloads and installs the latest pre-built rustx binary

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}RustX Installer${NC}"
echo "================="
echo ""

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        case "$ARCH" in
            x86_64)
                PLATFORM="linux-x86_64"
                ;;
            aarch64|arm64)
                PLATFORM="linux-aarch64"
                ;;
            *)
                echo -e "${RED}Unsupported architecture: $ARCH${NC}"
                exit 1
                ;;
        esac
        ;;
    Darwin*)
        case "$ARCH" in
            x86_64)
                PLATFORM="macos-x86_64"
                ;;
            arm64)
                PLATFORM="macos-aarch64"
                ;;
            *)
                echo -e "${RED}Unsupported architecture: $ARCH${NC}"
                exit 1
                ;;
        esac
        ;;
    *)
        echo -e "${RED}Unsupported OS: $OS${NC}"
        echo "This installer only supports Linux and macOS"
        echo "For Windows, please download manually from GitHub Releases"
        exit 1
        ;;
esac

echo -e "${GREEN}Detected platform:${NC} $PLATFORM"
echo ""

# Installation directory
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$INSTALL_DIR"

# Download URL
REPO="GrandpaEJx/RustX"
ARCHIVE_NAME="rustx-$PLATFORM.tar.gz"
DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/$ARCHIVE_NAME"

echo -e "${YELLOW}Downloading rustx...${NC}"
echo "Platform: $PLATFORM"

# Create temp directory
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# Download the archive
if command -v curl &> /dev/null; then
    curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$ARCHIVE_NAME" || {
        echo -e "${RED}Download failed.${NC}"
        echo "Please check:"
        echo "  1. Your internet connection"
        echo "  2. GitHub releases page: https://github.com/$REPO/releases"
        exit 1
    }
elif command -v wget &> /dev/null; then
    wget -q -O "$TMP_DIR/$ARCHIVE_NAME" "$DOWNLOAD_URL" || {
        echo -e "${RED}Download failed.${NC}"
        echo "Please check:"
        echo "  1. Your internet connection"
        echo "  2. GitHub releases page: https://github.com/$REPO/releases"
        exit 1
    }
else
    echo -e "${RED}Neither curl nor wget found. Please install one of them.${NC}"
    exit 1
fi

# Extract archive
cd "$TMP_DIR"
tar xzf "$ARCHIVE_NAME" || {
    echo -e "${RED}Failed to extract archive${NC}"
    exit 1
}

# Move to install directory
mv rustx "$INSTALL_DIR/rustx"

# Make executable
chmod +x "$INSTALL_DIR/rustx"

echo ""
echo -e "${GREEN}✓ RustX installed successfully!${NC}"
echo ""
echo -e "${YELLOW}Installation location:${NC} $INSTALL_DIR/rustx"

# Check if directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "${YELLOW}⚠ Warning:${NC} $INSTALL_DIR is not in your PATH"
    echo ""
    echo "Add it to your PATH by adding this line to your shell config:"
    echo ""
    echo -e "  ${BLUE}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
    echo ""
    echo "For bash: ~/.bashrc or ~/.bash_profile"
    echo "For zsh: ~/.zshrc"
    echo ""
    echo "Then run: source ~/.bashrc (or your config file)"
else
    echo ""
    echo -e "${GREEN}✓ Installation directory is already in PATH${NC}"
fi

echo ""
echo -e "${GREEN}Quick Start:${NC}"
echo "  rustx --help          # Show help"
echo "  rustx <script.rsx>    # Run a script"
echo "  rustx repl            # Start interactive REPL"
echo ""
echo -e "${BLUE}Documentation:${NC} https://github.com/$REPO"
echo ""
