# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NimbusCTL is a Terminal User Interface (TUI) application for managing AWS resources, built with Rust using the Ratatui framework. The project is currently in Phase 1 development with AWS SDK integration temporarily disabled.

## Development Commands

```bash
# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test

# Check for compilation errors
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt
```

## Architecture

### Core Structure

- **App State Management**: Centralized state in `src/app/state.rs` using `AppState` struct
- **Event Loop**: Async main loop handling keyboard input and UI updates in `src/main.rs`
- **Page-based Navigation**: Four main pages (Dashboard, ResourceList, ResourceDetail, Settings)
- **Multi-region AWS Support**: Architecture ready for AWS SDK integration in Phase 2

### Key Modules

- `src/app/` - Application state, configuration, and event handling
  - `state.rs` - Central AppState with navigation, AWS config, and UI state
  - `events.rs` - Input event processing and routing
- `src/aws/` - AWS service abstractions (Phase 2)
  - `client.rs` - Multi-region client management (stubs for Phase 1)
  - `services/` - Individual AWS service implementations
  - `types.rs` - AWS resource type definitions and service enums
- `src/ui/` - TUI components and rendering
  - `pages/` - Main application pages (dashboard, resource views, settings)
  - `components/` - Reusable UI components (header, selectors, notifications)
- `src/config/` - User configuration management
  - `user_config.rs` - TOML-based configuration with defaults
- `src/dashboard/` - Dashboard widgets and user favorites
- `src/utils/` - Error handling and helper utilities

### Navigation System

- **Quick Navigation**: Ctrl+P opens fuzzy search for AWS services
- **Page History**: Back navigation with Escape key
- **Service Types**: EC2, S3, RDS, IAM, Secrets Manager, EKS

### Configuration

- Config file: `~/.config/nimbus-ctl/config.toml`
- Auto-generated with sensible defaults on first run
- Supports AWS profiles, regions, display preferences, and dashboard layout

### Development Phase Notes

- **Phase 1**: UI foundation with mock AWS data (current)
- **Phase 2**: AWS SDK integration (commented out in Cargo.toml)
- AWS clients in `aws/client.rs` are currently stub implementations

### Key Controls

- `Ctrl+C`: Exit application
- `Ctrl+P`: Toggle quick navigation
- `?`: Toggle help panel
- `Esc`: Back/close dialogs
- Arrow keys: Navigation within lists and components
