curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# For your active (default) toolchain:
rustup component add rust-src

# If you also use nightly:
rustup component add rust-src --toolchain nightly
rustup component add rust-analyzer

echo 'source "$HOME/.cargo/env"' >> ~/.zshrc                                                          ─╯
exec $SHELL -l

zsh