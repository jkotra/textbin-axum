FROM rust:1.67 as build

RUN USER=root cargo new --bin textbin-axum
WORKDIR /textbin-axum

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# build
RUN cargo build --release

# run
FROM debian:buster-slim

# install deps
RUN apt-get update
RUN apt-get -y install openssl ca-certificates --no-install-recommends

EXPOSE 8000

ENV DATABASE_URL \
    GRC_SECRET \
    RUST_LOG="DEBUG"

# copy build artifact
COPY --from=build /textbin-axum/target/release/textbin-axum .

CMD ["./textbin-axum"]