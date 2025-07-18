# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NimbusCTL is a Terminal User Interface (TUI) application for managing AWS resources, built with Rust using the Ratatui framework. The project is currently in Phase 1 development with AWS SDK integration temporarily disabled but includes a comprehensive command system for future AWS operations.

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
- **Command System**: Context-aware command palette with service-specific actions
- **Multi-region AWS Support**: Architecture ready for AWS SDK integration in Phase 2

### Key Modules

- `src/app/` - Application state, configuration, and event handling
  - `state.rs` - Central AppState with navigation, AWS config, UI state, and command execution
  - `events.rs` - Input event processing and routing
- `src/command/` - Command system architecture
  - `commands.rs` - Core command structures and action definitions
  - `palette.rs` - Command palette UI and interaction logic
  - `context.rs` - Context-aware command filtering and state management
  - `registry/` - Service-specific command definitions (EC2, S3, RDS, IAM, EKS, Secrets)
- `src/aws/` - AWS service abstractions (Phase 2)
  - `client.rs` - Multi-region client management (stubs for Phase 1)
  - `services/` - Individual AWS service implementations
  - `types.rs` - AWS resource type definitions and service enums
- `src/ui/` - TUI components and rendering
  - `pages/dashboard/` - Dashboard page with favorites, recent activity, and widgets
  - `pages/` - Main application pages (resource views, settings)
  - `components/` - Reusable UI components (header, selectors, notifications, command palette)
- `src/config/` - User configuration management
  - `user_config.rs` - TOML-based configuration with defaults
- `src/utils/` - Error handling and helper utilities

### Command System

- **Command Palette**: `Ctrl+Shift+P` opens context-aware command palette
- **Context Awareness**: Commands are filtered based on current page, selected service, and resource
- **Service Commands**: Comprehensive AWS service operations (list, create, start, stop, delete, etc.)
- **Navigation Commands**: Quick navigation between services and pages
- **Profile/Region Commands**: Switch AWS profiles and regions dynamically

### Navigation System

- **Quick Navigation**: `Ctrl+P` opens fuzzy search for AWS services
- **Command Palette**: `Ctrl+Shift+P` opens context-aware command execution
- **Page History**: Back navigation with Escape key
- **Service Types**: EC2, S3, RDS, IAM, Secrets Manager, EKS

### Configuration

- Config file: `~/.config/nimbus-ctl/config.toml`
- Auto-generated with sensible defaults on first run
- Supports AWS profiles, regions, display preferences, and dashboard layout

### Development Phase Notes

- **Phase 1**: UI foundation with command system and mock AWS data (current)
- **Phase 2**: AWS SDK integration (commented out in Cargo.toml)
- AWS clients in `aws/client.rs` are currently stub implementations
- Command system includes placeholder implementations for all AWS service operations

### Key Controls

- `Ctrl+C`: Exit application
- `Ctrl+P`: Toggle quick navigation (service search)
- `Ctrl+Shift+P`: Toggle command palette (context-aware commands)
- `?`: Toggle help panel
- `Esc`: Back/close dialogs/command palette
- Arrow keys: Navigation within lists and components
- `Enter`: Execute selected command or navigate to resource

### Command Categories

- **Navigation**: Switch between pages and services
- **AWS Profiles**: Switch between configured AWS profiles
- **AWS Regions**: Switch between AWS regions
- **EC2**: Instance management (list, create, start, stop, terminate, reboot)
- **S3**: Bucket operations (list, create, delete, upload, download objects)
- **RDS**: Database management (list, start, stop, reboot, snapshot operations)
- **IAM**: User and role management (list, create, delete, policy operations)
- **Secrets Manager**: Secret operations (list, create, update, delete, retrieve values)
- **EKS**: Cluster management (list, create, delete, kubeconfig updates)
