FROM rust:1.31-slim

RUN mkdir /usr/src/app
WORKDIR /usr/src/app
RUN cargo install cargo-build-deps
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build-deps --release
COPY . .
RUN cargo check
RUN cargo test
