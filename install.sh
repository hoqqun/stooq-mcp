#!/bin/bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

REPO_URL="https://github.com/hoqqun/stooq-mcp.git"
INSTALL_DIR="${STOOQ_MCP_DIR:-$HOME/.stooq-mcp}"

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║       stooq-mcp Installer              ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo

# Detect if running from curl pipe or local
if [ -t 0 ] && [ -f "Cargo.toml" ]; then
    # Running locally in repo
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    INSTALL_DIR="$SCRIPT_DIR"
fi

BINARY_PATH="$INSTALL_DIR/target/release/stooq-mcp"

# Check if Rust is installed
check_rust() {
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: Rust is not installed.${NC}"
        echo "Please install Rust first: https://rustup.rs/"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    echo -e "${GREEN}✓${NC} Rust is installed"
}

# Clone repo if running from curl
clone_repo() {
    if [ ! -f "$INSTALL_DIR/Cargo.toml" ]; then
        echo -e "${YELLOW}Cloning stooq-mcp...${NC}"
        git clone "$REPO_URL" "$INSTALL_DIR"
        echo -e "${GREEN}✓${NC} Cloned to $INSTALL_DIR"
    else
        echo -e "${GREEN}✓${NC} Repository exists at $INSTALL_DIR"
    fi
}

# Build the project
build_project() {
    echo -e "${YELLOW}Building stooq-mcp...${NC}"
    cd "$INSTALL_DIR"
    cargo build --release
    echo -e "${GREEN}✓${NC} Build complete: $BINARY_PATH"
}

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Darwin*)    echo "macos" ;;
        Linux*)     echo "linux" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *)          echo "unknown" ;;
    esac
}

# Get Claude Desktop config path
get_claude_config_path() {
    local os=$(detect_os)
    case "$os" in
        macos)   echo "$HOME/Library/Application Support/Claude/claude_desktop_config.json" ;;
        linux)   echo "$HOME/.config/Claude/claude_desktop_config.json" ;;
        windows) echo "$APPDATA/Claude/claude_desktop_config.json" ;;
        *)       echo "" ;;
    esac
}

# Install to Claude Code
install_claude_code() {
    if ! command -v claude &> /dev/null; then
        echo -e "${YELLOW}Claude Code CLI not found. Skipping...${NC}"
        return 1
    fi
    
    echo -e "${YELLOW}Registering with Claude Code...${NC}"
    claude mcp add stooq-mcp "$BINARY_PATH" 2>/dev/null || {
        echo -e "${YELLOW}Already registered or failed. Try manually:${NC}"
        echo "  claude mcp add stooq-mcp $BINARY_PATH"
        return 1
    }
    echo -e "${GREEN}✓${NC} Registered with Claude Code"
    return 0
}

# Install to Claude Desktop
install_claude_desktop() {
    local config_path=$(get_claude_config_path)
    
    if [ -z "$config_path" ]; then
        echo -e "${YELLOW}Could not determine Claude Desktop config path${NC}"
        return 1
    fi
    
    local config_dir=$(dirname "$config_path")
    
    # Create config directory if needed
    if [ ! -d "$config_dir" ]; then
        echo -e "${YELLOW}Claude Desktop config directory not found. Skipping...${NC}"
        return 1
    fi
    
    # Create or update config
    if [ -f "$config_path" ]; then
        # Check if jq is available
        if command -v jq &> /dev/null; then
            # Backup existing config
            cp "$config_path" "$config_path.backup"
            
            # Add stooq-mcp to existing config
            jq --arg path "$BINARY_PATH" '.mcpServers["stooq-mcp"] = {"command": $path}' "$config_path" > "$config_path.tmp"
            mv "$config_path.tmp" "$config_path"
            echo -e "${GREEN}✓${NC} Updated Claude Desktop config"
            echo -e "  Backup saved: $config_path.backup"
        else
            echo -e "${YELLOW}jq not installed. Please add manually to:${NC}"
            echo "  $config_path"
            show_manual_config
            return 1
        fi
    else
        # Create new config
        cat > "$config_path" << EOF
{
  "mcpServers": {
    "stooq-mcp": {
      "command": "$BINARY_PATH"
    }
  }
}
EOF
        echo -e "${GREEN}✓${NC} Created Claude Desktop config"
    fi
    return 0
}

# Show manual configuration
show_manual_config() {
    echo
    echo -e "${BLUE}Manual Configuration:${NC}"
    echo "Add this to your Claude Desktop config:"
    echo
    echo '{'
    echo '  "mcpServers": {'
    echo '    "stooq-mcp": {'
    echo "      \"command\": \"$BINARY_PATH\""
    echo '    }'
    echo '  }'
    echo '}'
}

# Main menu
show_menu() {
    local is_pipe=false
    [ ! -t 0 ] && is_pipe=true

    echo
    
    # Claude Code
    if command -v claude &> /dev/null; then
        if $is_pipe; then
            echo -e "${YELLOW}Claude Code detected. Installing...${NC}"
            install_claude_code || true
        else
            read -p "Install to Claude Code? [Y/n]: " answer </dev/tty
            case "$answer" in
                [nN]*) echo "Skipped Claude Code" ;;
                *) install_claude_code || true ;;
            esac
        fi
    else
        echo -e "${YELLOW}Claude Code CLI not found. Skipping...${NC}"
    fi

    echo

    # Claude Desktop
    local config_path=$(get_claude_config_path)
    if [ -n "$config_path" ] && [ -d "$(dirname "$config_path")" ]; then
        if $is_pipe; then
            echo -e "${YELLOW}Claude Desktop detected. Installing...${NC}"
            install_claude_desktop || true
        else
            read -p "Install to Claude Desktop? [Y/n]: " answer </dev/tty
            case "$answer" in
                [nN]*) echo "Skipped Claude Desktop" ;;
                *) install_claude_desktop || true ;;
            esac
        fi
    else
        echo -e "${YELLOW}Claude Desktop not found. Skipping...${NC}"
    fi
}

# Main
main() {
    check_rust
    clone_repo
    build_project
    show_menu
    
    echo
    echo -e "${GREEN}════════════════════════════════════════${NC}"
    echo -e "${GREEN}Installation complete!${NC}"
    echo
    echo "Binary location: $BINARY_PATH"
    echo
    echo "Restart Claude Code/Desktop to use stooq-mcp."
    echo -e "${GREEN}════════════════════════════════════════${NC}"
}

main
