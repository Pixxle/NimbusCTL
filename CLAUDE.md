# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build and Run

```bash
cargo build          # Build the project
cargo run             # Run the TUI application
cargo check           # Check for compilation errors
```

### Development

```bash
cargo test            # Run tests (when implemented)
cargo clippy          # Run linter
cargo fmt             # Format code
```

## Architecture Overview

This is a Terminal User Interface (TUI) application for managing AWS resources, built with Rust and Ratatui. The application follows a dashboard-centric design with a Command-P style quick navigation system.

### Key Architectural Patterns

**Event-Driven TUI Architecture**: The application uses a main event loop in `main.rs` that handles terminal events and delegates to the `App` struct which manages state through `AppState`.

**Command-P Navigation**: Instead of traditional menu-based navigation, the application uses `Ctrl+P` to open a quick navigation overlay that allows users to search and jump directly to AWS services using fuzzy matching.

**Multi-Region AWS Client Management**: The `MultiRegionAwsClients` struct manages AWS SDK clients across multiple regions, allowing seamless switching between regions while maintaining connection state.

**Dashboard-Centric Design**: The default view is a dashboard with widgets showing favorites, recent activity, quick actions, and service overviews, rather than starting with a service list.

**Phase-Based Implementation**: Currently in Phase 1 (basic infrastructure), with AWS SDK integration temporarily disabled for initial TUI development.

### State Management

The application state is centralized in `AppState` which manages:

- Current page and navigation history
- AWS profile and region selection
- Quick navigation state (search input, suggestions, selection)
- Dashboard layout and widgets
- Favorites and recent activity
- UI visibility states for overlays

### Navigation System

**Page Flow**: Dashboard → ResourceList(ServiceType) → ResourceDetail(ServiceType, ResourceId) → Settings

- No dedicated Services page - replaced with Command-P quick nav
- Navigation uses `AppPage` enum with service-specific parameters

**Quick Navigation**:

- `Ctrl+P` opens search overlay
- Real-time fuzzy search across service names, descriptions, and keywords
- Arrow keys navigate suggestions, Enter selects, Escape cancels
- Implemented in `NavigationItem` and `NavigationAction` structs

### AWS Integration Architecture

**Service Structure**: Each AWS service (EC2, S3, RDS, IAM, Secrets, EKS) has its own module in `src/aws/services/` with consistent interfaces for CRUD operations.

**Profile Management**: AWS profiles are discovered from `~/.aws/credentials` and `~/.aws/config` using the `ProfileManager` struct.

**Resource Tagging**: Built-in support for AWS resource tagging through `TaggingService` for organizing and searching resources.

**Favorites System**: User-defined favorite resources persist across sessions using JSON storage in `FavoritesManager`.

### UI Component Architecture

**Modular Components**: UI components are split between:

- `ui/pages/` - Full-page views (dashboard, resource_list, resource_detail, settings)
- `ui/components/` - Reusable components (quick_nav, status_bar, help_panel, selectors)

**Overlay System**: Modal overlays (quick nav, help, selectors) use `Clear` widget and centered positioning.

**Responsive Layout**: Uses Ratatui's `Layout` with constraints to adapt to different terminal sizes.

### Configuration Management

**User Configuration**: Stored in TOML format with sections for AWS, display, behavior, and dashboard settings.

**Default Values**: Sensible defaults defined in `config/defaults.rs` including keybindings and dashboard widget configurations.

**Dashboard Customization**: Widget layout, enabled services, and display preferences are user-configurable.

### Error Handling

**Centralized Error Type**: Custom `AppError` enum in `utils/error.rs` handles all error scenarios with specific variants for AWS, IO, parsing, and general errors.

**Async Error Propagation**: Uses `anyhow::Result` for async functions and custom `Result<T>` type alias for consistent error handling.

### Development Notes

**AWS SDK Status**: AWS SDK dependencies are currently commented out in `Cargo.toml` (Phase 1 implementation). Phase 2 will re-enable them for real AWS integration.

**Mock Data**: Current implementation uses mock data for services and resources to enable UI development without AWS dependencies.

**Keyboard-First Design**: All interactions are keyboard-driven with intuitive shortcuts. No mouse support by design.

### Key Files

- `src/app/state.rs` - Central application state management
- `src/ui/components/quick_nav.rs` - Command-P navigation implementation
- `src/aws/client.rs` - Multi-region AWS client management
- `src/dashboard/favorites.rs` - Favorites persistence and management
- `src/config/user_config.rs` - User configuration structure
- `src/utils/error.rs` - Centralized error handling

### Testing Strategy

The application is designed for integration testing of the complete TUI workflow. Unit tests should focus on:

- AWS service integration logic
- Configuration loading/saving
- Navigation state management
- Search/filtering algorithms
