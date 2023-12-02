FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Change the user to vscode to avoid permission issues when installing packages.
USER vscode 

RUN cargo install trunk \
    && rustup toolchain install nightly \
    && rustup default nightly \
    && rustup target add wasm32-unknown-unknown