#!/bin/bash
# Claude Code Setup - macOS Installer
# Downloads and installs the latest release for macOS x64
# Usage: bash install-macos.sh [--install-dir /custom/path]

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Config
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
SKIP_CONFIG=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --skip-config)
            SKIP_CONFIG=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo -e "${CYAN}🚀 Claude Code Setup - macOS Installer${NC}"
echo -e "${CYAN}📦 Installing to: $INSTALL_DIR${NC}"

# Fetch latest release
echo -e "${YELLOW}📥 Fetching latest release...${NC}"
API_RESPONSE=$(curl -s "https://api.github.com/repos/Ercaner1988/claude-code-setup-rustified/releases/latest")
LATEST_TAG=$(echo "$API_RESPONSE" | grep -o '"tag_name": *"[^"]*' | head -1 | cut -d'"' -f4)

if [[ -z "$LATEST_TAG" ]]; then
    echo -e "${RED}❌ Could not fetch latest release${NC}"
    exit 1
fi

echo -e "${GREEN}   Found: $LATEST_TAG${NC}"

# Pick the asset for THIS operating system. Bu betik hem macOS hem Linux icin
# kullaniliyor; macOS ikilisini sabitlemek Linux'ta "cannot execute binary
# file" hatasi veriyordu.
case "$(uname -s)" in
    Darwin) ASSET_MARKER="macos-x86_64"; OS_LABEL="macOS" ;;
    Linux)  ASSET_MARKER="linux-x86_64"; OS_LABEL="Linux" ;;
    *)
        echo "Unsupported operating system: $(uname -s)"
        echo "Supported: macOS, Linux. On Windows use install-windows.ps1."
        exit 1
        ;;
esac

DOWNLOAD_URL=$(echo "$API_RESPONSE" | grep -o "\"browser_download_url\": *\"[^\"]*${ASSET_MARKER}[^\"]*" | head -1 | cut -d'"' -f4)

if [[ -z "$DOWNLOAD_URL" ]]; then
    echo "${OS_LABEL} x64 binary not found in release"
    exit 1
fi

BINARY_NAME=$(basename "$DOWNLOAD_URL")
echo -e "${GREEN}   Binary: $BINARY_NAME${NC}"

# Create install directory
mkdir -p "$INSTALL_DIR"
echo -e "${GREEN}✅ Created directory: $INSTALL_DIR${NC}"

# Download binary
BINARY_PATH="$INSTALL_DIR/claude-code-setup"
echo -e "${YELLOW}⬇️  Downloading binary...${NC}"
curl -L "$DOWNLOAD_URL" -o "$BINARY_PATH"
echo -e "${GREEN}✅ Downloaded: $BINARY_PATH${NC}"

# Make executable
chmod +x "$BINARY_PATH"
echo -e "${GREEN}✅ Made executable${NC}"

# Add to PATH if needed
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo -e "${YELLOW}🔧 Adding $INSTALL_DIR to PATH...${NC}"
    
    SHELL_RC=""
    if [[ -f "$HOME/.zshrc" ]]; then
        SHELL_RC="$HOME/.zshrc"
    elif [[ -f "$HOME/.bash_profile" ]]; then
        SHELL_RC="$HOME/.bash_profile"
    fi
    
    if [[ -n "$SHELL_RC" && ! $(grep -q "$INSTALL_DIR" "$SHELL_RC" 2>/dev/null || echo "not found") ]]; then
        echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$SHELL_RC"
        echo -e "${GREEN}✅ Added to $SHELL_RC${NC}"
    fi
fi

# Run install command (optional config)
if [[ "$SKIP_CONFIG" != "true" ]]; then
    echo ""
    echo -e "${YELLOW}⚙️  Running setup...${NC}"
    "$BINARY_PATH" install || true
fi

echo ""
echo -e "${GREEN}✅ Installation complete!${NC}"
echo ""
echo -e "${CYAN}📝 Next steps:${NC}"
echo -e "${CYAN}   1. Restart Terminal or run: source ~/.zshrc${NC}"
echo -e "${CYAN}   2. Verify: claude-code-setup --version${NC}"
echo -e "${CYAN}   3. Configure MCP: claude-code-setup mcp-list${NC}"
echo ""
echo -e "${CYAN}📚 Docs: https://github.com/Ercaner1988/claude-code-setup-rustified${NC}"
