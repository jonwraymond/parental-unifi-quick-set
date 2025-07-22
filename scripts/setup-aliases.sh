#!/bin/bash

# Setup useful git aliases for frequent commits

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}🔧 Setting up Git Aliases${NC}"
echo "=========================="

# Quick commit aliases
echo -e "${BLUE}📝 Adding git aliases...${NC}"

git config alias.qc '!./scripts/quick-commit.sh'
git config alias.ac 'add -A && git commit -m'
git config alias.acp '!f() { git add -A && git commit -m "$1" && git push; }; f'
git config alias.st 'status --short'
git config alias.lg 'log --oneline --graph --decorate -10'
git config alias.undo 'reset HEAD~1'
git config alias.amend 'commit --amend --no-edit'

echo -e "${GREEN}✅ Git aliases configured!${NC}"
echo ""
echo -e "${BLUE}📋 Available aliases:${NC}"
echo "  git qc                    - Quick commit (interactive)"
echo "  git qc \"message\"          - Quick commit with message"
echo "  git ac \"message\"          - Add all and commit"
echo "  git acp \"message\"         - Add all, commit, and push"
echo "  git st                    - Short status"
echo "  git lg                    - Pretty log (last 10)"
echo "  git undo                  - Undo last commit (keep changes)"
echo "  git amend                 - Amend last commit"
echo ""
echo -e "${YELLOW}💡 Examples:${NC}"
echo "  git qc \"feat: Add new feature\""
echo "  git acp \"fix: Quick bug fix\""
echo "  git st && git lg" 