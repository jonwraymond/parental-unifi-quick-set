#!/bin/bash

# Quick commit script for frequent, descriptive commits
# Usage: ./scripts/quick-commit.sh "feature: description" or just ./scripts/quick-commit.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to show current status
show_status() {
    echo -e "${BLUE}📋 Current Git Status:${NC}"
    git status --short
    echo ""
}

# Function to show recent commits
show_recent_commits() {
    echo -e "${BLUE}📈 Recent Commits:${NC}"
    git log --oneline -5
    echo ""
}

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}❌ Error: Not in a git repository${NC}"
    exit 1
fi

echo -e "${GREEN}🚀 Quick Commit Script${NC}"
echo "=========================="

show_status

# Check if there are any changes to commit
if git diff --quiet && git diff --cached --quiet; then
    echo -e "${YELLOW}✅ No changes to commit${NC}"
    show_recent_commits
    exit 0
fi

# Get commit message
if [ -n "$1" ]; then
    COMMIT_MSG="$1"
else
    echo -e "${YELLOW}💬 Enter commit message (or press Enter for auto-generated):${NC}"
    read -r COMMIT_MSG
fi

# Auto-generate commit message if empty
if [ -z "$COMMIT_MSG" ]; then
    # Get list of modified files
    MODIFIED_FILES=$(git diff --name-only --cached 2>/dev/null || git diff --name-only)
    
    if [ -z "$MODIFIED_FILES" ]; then
        COMMIT_MSG="Update: Minor changes"
    else
        FILE_COUNT=$(echo "$MODIFIED_FILES" | wc -l | tr -d ' ')
        if [ "$FILE_COUNT" -eq 1 ]; then
            FILE_TYPE=$(echo "$MODIFIED_FILES" | head -1 | grep -o '\.[^.]*$' | tr -d '.')
            case $FILE_TYPE in
                rs) COMMIT_MSG="Update Rust code: $(basename "$MODIFIED_FILES")" ;;
                toml) COMMIT_MSG="Update config: $(basename "$MODIFIED_FILES")" ;;
                md) COMMIT_MSG="Update documentation: $(basename "$MODIFIED_FILES")" ;;
                yml|yaml) COMMIT_MSG="Update workflow: $(basename "$MODIFIED_FILES")" ;;
                html) COMMIT_MSG="Update UI: $(basename "$MODIFIED_FILES")" ;;
                *) COMMIT_MSG="Update: $(basename "$MODIFIED_FILES")" ;;
            esac
        else
            COMMIT_MSG="Update: Multiple files ($FILE_COUNT files)"
        fi
    fi
fi

# Stage all changes if nothing is staged
if git diff --cached --quiet; then
    echo -e "${BLUE}📦 Staging all changes...${NC}"
    git add .
fi

# Show what will be committed
echo -e "${BLUE}📋 Files to be committed:${NC}"
git diff --cached --name-status

echo ""
echo -e "${YELLOW}📝 Commit message: \"$COMMIT_MSG\"${NC}"
echo ""

# Ask for confirmation
read -p "Proceed with commit? (Y/n): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Nn]$ ]]; then
    echo -e "${YELLOW}❌ Commit cancelled${NC}"
    exit 1
fi

# Commit the changes
echo -e "${BLUE}💾 Committing changes...${NC}"
git commit -m "$COMMIT_MSG"

echo -e "${GREEN}✅ Committed successfully!${NC}"

# Ask about pushing
read -p "Push to origin? (Y/n): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Nn]$ ]]; then
    echo -e "${BLUE}🚀 Pushing to origin...${NC}"
    git push origin "$(git branch --show-current)"
    echo -e "${GREEN}✅ Pushed successfully!${NC}"
fi

echo ""
show_recent_commits 