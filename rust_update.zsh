#!/usr/bin/env bash
set -euo pipefail

REQUIRED_VERSION="1.75.0"

# Function to compare Rust versions
version_ge() {
    # Returns 0 (true) if $1 >= $2
    [ "$(printf '%s\n' "$2" "$1" | sort -V | head -n1)" = "$2" ]
}

INSTALL_RUSTUP=false

if command -v rustc &>/dev/null; then
    INSTALLED_VERSION=$(rustc --version | awk '{print $2}')
    if version_ge "$INSTALLED_VERSION" "$REQUIRED_VERSION"; then
        echo "✅ Rust $INSTALLED_VERSION is installed (meets requirement)."
    else
        echo "⚠️ Rust $INSTALLED_VERSION is older than $REQUIRED_VERSION. Updating..."
        INSTALL_RUSTUP=true
    fi
else
    echo "❌ Rust not found. Installing..."
    INSTALL_RUSTUP=true
fi

# Install or update Rust if needed
if $INSTALL_RUSTUP; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    rustup update stable
fi

# Ensure rust-src for stable
rustup component add rust-src || true

# If nightly exists, ensure rust-src for nightly
if rustup toolchain list | grep -q nightly; then
    rustup component add rust-src --toolchain nightly || true
fi

# Ensure rust-analyzer component
if ! rustup component list | grep -q "rust-analyzer.*installed"; then
    rustup component add rust-analyzer || true
fi

# Add cargo env to shell rc if missing
SHELL_RC="$HOME/.zshrc"
if ! grep -q 'source "$HOME/.cargo/env"' "$SHELL_RC"; then
    echo 'source "$HOME/.cargo/env"' >> "$SHELL_RC"
    echo "Added Cargo env sourcing to $SHELL_RC"
fi

echo "🎉 Rust setup complete. Restarting shell..."
exec "$SHELL" -l
