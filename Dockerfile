FROM rust:1.99-slim
MAINTAINER Jan Burkhardt <264754-bjrke@users.noreply.gitlab.com>
RUN rustup component add rustfmt clippy
RUN apt update
RUN apt install -y curl
RUN curl -sSfL https://deb.nodesource.com/setup_24.x | bash -
RUN apt install -y nodejs
RUN curl -sSL -o /tmp/wasm-pack.tar.gz https://github.com/wasm-bindgen/wasm-pack/releases/download/v0.15.0/wasm-pack-v0.15.0-x86_64-unknown-linux-musl.tar.gz \
    && tar -xzf /tmp/wasm-pack.tar.gz -C /usr/local/bin --strip-components=1 wasm-pack-v0.15.0-x86_64-unknown-linux-musl/wasm-pack \
    && rm /tmp/wasm-pack.tar.gz
