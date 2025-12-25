# YouGile Rust - API Client and Terminal UI

A complete Rust solution for YouGile project management:
- **yougile-client**: Low-level REST API client (auto-generated from OpenAPI spec)
- **yougile-sdk**: High-level SDK with builder patterns and convenience methods
- **yougile-tui**: Terminal UI for managing projects and tasks

## Quick Start

### Prerequisites

- Rust 1.70+ (install via [rustup.rs](https://rustup.rs/) or NixOS)
- YouGile API token from https://yougile.com

### Setup with NixOS (Recommended)

If you're using NixOS or have `direnv` installed:

```bash
# Clone the repo
git clone <repo-url>
cd yougile-rs

# Allow direnv to load the Nix environment
direnv allow

# Or manually load the flake
nix flake update
nix develop
```

### Setup Without Nix

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone <repo-url>
cd yougile-rs
cargo build
```

## Using the TUI Application

### Configuration

Set your YouGile API credentials:

**Option 1: Environment Variables**
```bash
export YOUGILE_API_URL="https://api.yougile.com"
export YOUGILE_API_TOKEN="your_token_here"
export RUST_LOG="info"  # optional: debug, trace
```

**Option 2: Config File**
```bash
mkdir -p ~/.config/yougile-tui
cat > ~/.config/yougile-tui/config.toml << EOF
api_url = "https://api.yougile.com"
api_token = "your_token_here"
EOF
```

**Option 3: .env.local (with direnv)**
```bash
cp .env.example .env.local
# Edit .env.local with your credentials
```

### Running the TUI

```bash
# With NixOS/direnv (environment auto-loaded)
cargo run -p yougile-tui

# Or with explicit env vars
YOUGILE_API_TOKEN="your_token" cargo run -p yougile-tui
```

### Controls

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `Tab` | Switch focus |
| `Enter` | Select item |
| `r` | Refresh |
| `p` | Projects view |
| `h` | Help |
| `q` / `Esc` | Quit |

## Development

### Using the SDK

```rust
use yougile_sdk::YouGileClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let client = YouGileClient::new(
        "https://api.yougile.com",
        "your_token"
    );
    
    let projects = client.get_projects().await?;
    for project in projects {
        println!("Project: {:?}", project.title);
    }
    
    Ok(())
}
```

### Project Structure

```
yougile-rs/
├── yougile-client/          # Low-level API client (OpenAPI generated)
│   ├── src/
│   │   ├── apis/           # API endpoints
│   │   ├── models/         # Data models
│   │   └── lib.rs
│   └── Cargo.toml
│
├── yougile-sdk/             # High-level SDK
│   ├── src/
│   └── Cargo.toml
│
├── yougile-tui/             # Terminal UI application
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── app.rs          # State management
│   │   ├── ui.rs           # Rendering
│   │   ├── api.rs          # SDK integration
│   │   ├── handlers.rs     # Event handlers
│   │   └── config.rs       # Configuration
│   └── Cargo.toml
│
├── flake.nix                # Nix development environment
├── .envrc                   # direnv configuration
├── .env.example             # Environment template
└── Cargo.toml              # Workspace definition
```

### Development Commands

```bash
# Build all packages
cargo build

# Build TUI for release
cargo build -p yougile-tui --release

# Run tests
cargo test

# Format code
cargo fmt

# Check with clippy
cargo clippy -- -D warnings

# Run TUI with debug logging
RUST_LOG=debug cargo run -p yougile-tui
```

## Features

### yougile-client
- Auto-generated from YouGile OpenAPI v2.0 specification
- Complete REST API coverage
- Async/await with tokio
- Configurable TLS backends (native-tls, rustls-tls)

### yougile-sdk
- Higher-level convenience methods
- Builder pattern for complex operations
- Integrated error handling
- Logging support

### yougile-tui
- View projects and tasks
- Real-time project list
- Configuration management
- Error handling and user feedback
- Cross-platform terminal support

## Logging

Control verbosity with `RUST_LOG`:

```bash
# Show only errors
RUST_LOG=error cargo run -p yougile-tui

# Show detailed debug info
RUST_LOG=debug cargo run -p yougile-tui

# Show everything
RUST_LOG=trace cargo run -p yougile-tui

# Only YouGile logs
RUST_LOG=yougile=debug cargo run -p yougile-tui
```

## Troubleshooting

### "Failed to load configuration"

Make sure environment variables are set:
```bash
export YOUGILE_API_URL="https://api.yougile.com"
export YOUGILE_API_TOKEN="your_actual_token"
```

### "API client not initialized"

Check that your token is valid and the API URL is correct.

### Build issues on NixOS

Make sure to load the Nix environment:
```bash
nix flake update
nix develop
```

## License

MIT

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a pull request
