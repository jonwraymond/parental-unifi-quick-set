#!/bin/bash

# Install git hooks for the project

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_HOOKS_DIR="$SCRIPT_DIR/git-hooks"
LOCAL_HOOKS_DIR=".git/hooks"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}🔧 Installing Git Hooks${NC}"
echo "========================"

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo -e "${RED}❌ Error: Not in a git repository${NC}"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p "$LOCAL_HOOKS_DIR"

# Install pre-commit hook
if [ -f "$GIT_HOOKS_DIR/pre-commit" ]; then
    echo -e "${BLUE}📋 Installing pre-commit hook...${NC}"
    cp "$GIT_HOOKS_DIR/pre-commit" "$LOCAL_HOOKS_DIR/pre-commit"
    chmod +x "$LOCAL_HOOKS_DIR/pre-commit"
    echo -e "${GREEN}✅ Pre-commit hook installed${NC}"
else
    echo -e "${YELLOW}⚠️  Pre-commit hook not found${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Git hooks installation complete!${NC}"
echo ""
echo -e "${BLUE}📝 Available tools:${NC}"
echo "  • ./scripts/quick-commit.sh - Fast, interactive commits"
echo "  • Pre-commit hook - Warns about large commits"
echo ""
echo -e "${YELLOW}💡 Tip: Use ./scripts/quick-commit.sh for easy frequent commits${NC}" 