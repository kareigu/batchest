FROM rust as rust_builder

WORKDIR /usr/src

RUN USER=root cargo new --bin batchest

WORKDIR /usr/src/batchest

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm ./target/release/deps/batchest*
RUN rm src/*.rs

COPY ./src ./src
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/src/batchest

RUN apt-get update

COPY --from=rust_builder /usr/src/batchest/target/release/batchest ./batchest

CMD ./batchest
