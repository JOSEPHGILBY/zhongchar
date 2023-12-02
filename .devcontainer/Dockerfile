FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN cargo install trunk \
    && rustup toolchain install nightly \
    && rustup default nightly 
    && rustup target add wasm32-unknown-unknown