{
  description = "YouGile Rust API Client and TUI Application";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };

        nativeBuildInputs = with pkgs; [
          rust
          cargo
          rustfmt
          clippy
          pkg-config
          openssl
          direnv
        ];

        buildInputs = with pkgs; [
          # Terminal/TUI dependencies
          openssl
          libxcb
          libxkbcommon
          libGL

          # For development
          git
          cargo-edit
          cargo-outdated
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            echo "YouGile Rust Project Development Environment"
            echo "================================================"
            echo ""
            echo "Available commands:"
            echo "  cargo build           - Build the project"
            echo "  cargo build -p yougile-tui --release - Build TUI app for release"
            echo "  cargo run -p yougile-tui - Run TUI application"
            echo "  cargo test            - Run tests"
            echo "  cargo fmt             - Format code"
            echo "  cargo clippy          - Run linter"
            echo "  cargo doc --open      - Build and open documentation"
            echo ""
            echo "Environment variables for TUI:"
            echo "  YOUGILE_API_URL       - API endpoint"
            echo "  YOUGILE_API_TOKEN     - Your API token (REQUIRED)"
            echo "  RUST_LOG              - Log level: error, warn, info, debug, trace"
            echo ""
            echo "Setup:"
            echo "  1. Set YOUGILE_API_TOKEN: export YOUGILE_API_TOKEN=\"your_token\""
            echo "  2. Run TUI: cargo run -p yougile-tui"
            echo ""
          '';

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          RUST_BACKTRACE = 1;
        };
      }
    );
}
