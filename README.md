# NimbusCTL

A Terminal User Interface (TUI) for managing AWS resources, built with Rust.

## Installation

### Prerequisites

- Rust 1.70 or later
- AWS CLI configured with appropriate credentials

### Build from Source

```bash
git clone https://github.com/your-username/nimbus-ctl.git
cd nimbus-ctl
cargo build --release
```

### Run

```bash
cargo run
```

## Development

```bash
# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Run linter
cargo clippy
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the TUI framework
- Uses [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust) for AWS integration
- Inspired by tools like `k9s` and `lazygit`
