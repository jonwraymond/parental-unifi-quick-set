# Git Commit Guidelines

This document outlines best practices for making frequent, meaningful commits that enable easy rollback and better version control.

## 🎯 Why Frequent Commits Matter

- **Easy Rollback**: Small commits make it easier to revert specific changes
- **Better History**: Clear progression of development
- **Easier Reviews**: Smaller changes are easier to review
- **Reduced Risk**: Less likely to lose work or introduce bugs
- **Better Collaboration**: Easier for team members to follow progress

## 📝 Commit Message Format

Use this format for consistent, informative commit messages:

```
<type>: <description>

[optional body]
```

### Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `build`: Build system or dependency changes
- `ci`: CI/CD changes
- `perf`: Performance improvements
- `chore`: Maintenance tasks

### Examples:
```bash
feat: Add recurring schedule support for app blocking
fix: Resolve authentication token expiration issue
docs: Update README with deployment instructions
refactor: Extract UniFi API client into separate module
test: Add unit tests for rule creation logic
```

## 🚀 Quick Commit Tools

### 1. Quick Commit Script
Use the provided script for fast, interactive commits:

```bash
# Interactive mode (will prompt for message)
./scripts/quick-commit.sh

# With message
./scripts/quick-commit.sh "feat: Add new blocking rule type"

# Auto-generated message (based on files changed)
./scripts/quick-commit.sh ""
```

### 2. Install Git Hooks
Set up helpful pre-commit warnings:

```bash
./scripts/install-hooks.sh
```

## 📏 Commit Size Guidelines

### Ideal Commit Size:
- **1-5 files** changed per commit
- **50-200 lines** changed per commit
- **One logical change** per commit

### When to Split Commits:

#### Split by Type:
```bash
# Instead of one large commit:
git commit -m "Add feature and update docs"

# Split into:
git commit -m "feat: Add recurring schedule feature"
git commit -m "docs: Update README with new feature info"
```

#### Split by Component:
```bash
# Instead of:
git commit -m "Update frontend and backend"

# Split into:
git commit -m "feat: Add schedule picker component"
git commit -m "feat: Add backend support for recurring schedules"
```

#### Split by Scope:
```bash
# Instead of:
git commit -m "Fix multiple bugs"

# Split into:
git commit -m "fix: Resolve authentication timeout issue"
git commit -m "fix: Handle empty device list gracefully"
git commit -m "fix: Correct time zone handling in schedules"
```

## 🔄 Common Workflows

### Feature Development:
```bash
# 1. Start with basic structure
git commit -m "feat: Add basic schedule rule structure"

# 2. Add core functionality
git commit -m "feat: Implement schedule validation logic"

# 3. Add error handling
git commit -m "feat: Add error handling for invalid schedules"

# 4. Add tests
git commit -m "test: Add unit tests for schedule validation"

# 5. Update documentation
git commit -m "docs: Document new schedule feature"
```

### Bug Fixing:
```bash
# 1. Add failing test (if applicable)
git commit -m "test: Add test case for token expiration bug"

# 2. Fix the bug
git commit -m "fix: Handle token expiration in API client"

# 3. Update related code
git commit -m "refactor: Improve error messaging for auth failures"
```

### Refactoring:
```bash
# 1. Extract function/module
git commit -m "refactor: Extract API client to separate module"

# 2. Update imports/usage
git commit -m "refactor: Update imports after API client extraction"

# 3. Clean up unused code
git commit -m "refactor: Remove unused helper functions"
```

## 🛡️ Safety Practices

### Before Committing:
1. **Review changes**: `git diff --cached`
2. **Run tests**: `cargo test` (if applicable)
3. **Check formatting**: `cargo fmt --check`
4. **Verify build**: `cargo check`

### Rollback Strategies:

#### Undo Last Commit (keep changes):
```bash
git reset HEAD~1
```

#### Undo Last Commit (discard changes):
```bash
git reset --hard HEAD~1
```

#### Revert Specific Commit:
```bash
git revert <commit-hash>
```

#### Interactive Rebase (reorder/edit commits):
```bash
git rebase -i HEAD~5
```

## 🏃‍♂️ Quick Reference

### Daily Workflow:
```bash
# Check status
git status

# Quick commit with script
./scripts/quick-commit.sh

# Or manual commit
git add .
git commit -m "feat: Add description"
git push
```

### When in Doubt:
- **Smaller is better** - err on the side of more frequent commits
- **One idea per commit** - if you're using "and" in your commit message, consider splitting
- **Test before committing** - each commit should leave the code in a working state
- **Descriptive messages** - write for your future self and teammates

## 🎉 Benefits You'll See

With frequent commits, you'll experience:
- ✅ Easier debugging (pinpoint when issues were introduced)
- ✅ Faster code reviews
- ✅ More confidence when refactoring
- ✅ Better team collaboration
- ✅ Cleaner git history
- ✅ Easier rollbacks when needed

Remember: **Commit early, commit often!** 🚀 