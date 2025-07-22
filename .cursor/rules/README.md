# Cursor Rules for Parental UniFi Quick Set

This directory contains Cursor rules that provide context and guidance for development of the Parental UniFi Quick Set application.

## 📋 Available Rules

### 🏗️ **project-structure.mdc** (Always Applied)
- **Scope**: All files
- **Purpose**: Provides high-level project overview and architecture understanding
- **Contains**: Key files, project purpose, deployment options, architecture diagram

### 🔌 **unifi-api-guidelines.mdc** (Manual/Search)
- **Scope**: When working with UniFi API code
- **Purpose**: Critical authentication and API integration requirements
- **Contains**: Local vs Site Manager API differences, authentication setup, common mistakes

### 🦀 **rust-axum-patterns.mdc** (Rust Files)
- **Scope**: `*.rs` files
- **Purpose**: Rust and Axum framework patterns specific to this project
- **Contains**: State management, error handling, UniFi integration patterns, dependencies

### 📝 **git-commit-standards.mdc** (Manual/Search)
- **Scope**: When working with git commits
- **Purpose**: Commit message standards and workflow tools
- **Contains**: Conventional commits, available git tools, commit frequency philosophy

### 🚀 **deployment-config.mdc** (Config Files)
- **Scope**: `Dockerfile`, `docker-compose.yml`, `fly.toml`
- **Purpose**: Deployment configuration patterns and best practices
- **Contains**: Multi-stage Docker builds, cloud deployment, security notes

## 🎯 How Rules Are Applied

### Automatic Rules
- **project-structure.mdc**: Always loaded for every request

### File-Specific Rules  
- **rust-axum-patterns.mdc**: Applied when editing `.rs` files
- **deployment-config.mdc**: Applied when editing deployment configuration files

### Manual Rules
- **unifi-api-guidelines.mdc**: Available when searching for UniFi API help
- **git-commit-standards.mdc**: Available when searching for git workflow help

## 🔍 Using the Rules

Rules help Cursor understand:
- **Project context** and architecture
- **Code patterns** and conventions specific to this project
- **UniFi API requirements** and authentication methods
- **Deployment strategies** and configuration
- **Git workflow** and commit standards

This enables more accurate code suggestions, better troubleshooting, and context-aware assistance.

## 📚 Related Documentation

- [UniFi Setup Guide](../docs/UNIFI_SETUP_GUIDE.md) - Complete setup instructions
- [Commit Guidelines](../COMMIT_GUIDELINES.md) - Detailed git workflow
- [Main README](../README.md) - Project overview and quick start 