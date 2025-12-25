# YouGile TUI

Terminal User Interface for YouGile project management.

## Building

```bash
cargo build --release -p yougile-tui
```

## Running

First, set up your configuration via environment variables:

```bash
export YOUGILE_API_URL="https://api.yougile.com"
export YOUGILE_API_TOKEN="your_token_here"

cargo run -p yougile-tui
```

Or create a config file at `~/.config/yougile-tui/config.toml`:

```toml
api_url = "https://api.yougile.com"
api_token = "your_token_here"
```

## Controls

### Navigation
- `j` / `↓` - Move down
- `k` / `↑` - Move up
- `Tab` - Switch focus between panels

### Actions
- `Enter` - Select/Open item
- `r` - Refresh data
- `p` - Projects view
- `h` - Help
- `q` / `Esc` - Quit

## Architecture

- `main.rs` - Event loop and terminal setup
- `app.rs` - Application state management
- `ui.rs` - UI rendering using ratatui
- `handlers.rs` - Event handlers (async operations)
- `config.rs` - Configuration loading/saving

## Development

The TUI uses ratatui for rendering and crossterm for terminal handling. The app state is managed in `app.rs` and can be extended to support more features.

To add new views or features:

1. Add new variant to `View` enum in `app.rs`
2. Add handler in `handle_key_event()` method
3. Add rendering function in `ui.rs`
4. Implement API calls in `handlers.rs`
